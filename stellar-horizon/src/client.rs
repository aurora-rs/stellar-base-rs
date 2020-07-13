use crate::error::Result;
use futures::future::{BoxFuture, Future};
use futures::stream::{BoxStream, IntoAsyncRead};
use futures::stream::{StreamExt, TryStreamExt};
use futures::Stream;
use hyper::client::ResponseFuture;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::de::{Deserialize, DeserializeOwned};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub trait Request: Send + Sync {
    type Response: DeserializeOwned;

    fn is_post(&self) -> bool {
        false
    }

    fn uri(&self, host: &str) -> Result<String>;
}

pub trait StreamRequest: Request + std::marker::Unpin {
    type Resource: DeserializeOwned + Send + Sync + std::marker::Unpin;
}

#[derive(Debug, Deserialize)]
pub struct Ledger {
    sequence: i64,
}

pub trait HorizonClient {
    fn request<'a, R: Request>(&'a self, req: &'a R) -> BoxFuture<'a, Result<R::Response>>;
    fn stream<'a, R: StreamRequest>(
        &'a self,
        req: R,
    ) -> Result<Box<dyn Stream<Item = Result<R::Resource>> + 'a + std::marker::Unpin>>
    where
        R: 'a,
        //R::Resource: 'a + Send + Sync + std::marker::Unpin;
        R::Resource: 'a;
}

type HttpClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct HorizonHttpClient {
    inner: HttpClient,
    host: http::Uri,
}

type BoxDecoder = Box<dyn Unpin + Stream<Item = http_types::Result<async_sse::Event>>>;

pub struct HorizonHttpStream<'a, R>
where
    R: StreamRequest,
{
    client: &'a HorizonHttpClient,
    last_id: Option<String>,
    uri: http::Uri,
    request: R,
    response: Option<ResponseFuture>,
    decoder: Option<BoxDecoder>,
}

impl HorizonHttpClient {
    pub fn new(host: &str) -> Result<HorizonHttpClient> {
        let https = HttpsConnector::new();
        let inner = Client::builder().build::<_, hyper::Body>(https);
        let host: http::Uri = host.parse()?;
        Ok(HorizonHttpClient { inner, host })
    }
}

impl HorizonClient for HorizonHttpClient {
    fn request<'a, R: Request>(&'a self, req: &'a R) -> BoxFuture<'a, Result<R::Response>> {
        Box::pin(execute_request(self, req))
    }

    fn stream<'a, 'b, R: StreamRequest>(
        &'a self,
        req: R,
    ) -> Result<Box<dyn Stream<Item = Result<R::Resource>> + 'a + std::marker::Unpin>>
    where
        //    R::Resource: 'a + Send + Sync + std::marker::Unpin,
        R: 'a,
        R::Resource: 'a,
    {
        let uri_str = req.uri(&self.host.to_string())?;
        let uri_str_str = format!("{}?cursor=now", uri_str);
        let uri: http::Uri = uri_str_str.parse()?;
        let phantom: PhantomData<R> = PhantomData;
        Ok(Box::new(HorizonHttpStream {
            client: &self,
            last_id: None,
            uri,
            response: None,
            decoder: None,
            request: req,
        }))
    }
}

async fn execute_request<R: Request>(client: &HorizonHttpClient, req: &R) -> Result<R::Response> {
    let http_method = if req.is_post() {
        hyper::Method::POST
    } else {
        hyper::Method::GET
    };
    let uri = req.uri(&client.host.to_string())?;
    let request = hyper::Request::builder()
        .method(http_method)
        .uri(uri)
        .body(hyper::Body::empty())?;

    let response = client.inner.request(request).await?;

    if response.status().is_success() {
        let bytes = hyper::body::to_bytes(response).await?;
        let result: R::Response = serde_json::from_slice(&bytes)?;
        Ok(result)
    } else {
        // Parse Error response
        todo!()
    }
}

impl<'a, R> Stream for HorizonHttpStream<'a, R>
where
    R: StreamRequest,
{
    type Item = Result<R::Resource>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            if self.response.is_none() && self.decoder.is_none() {
                let mut request_builder = http::Request::get(&self.uri);
                request_builder = request_builder.header("Accept", "text/event-stream");
                if let Some(last_id) = &self.last_id {
                    request_builder = request_builder.header("Last-Event-Id", last_id.clone());
                }

                println!("send request");
                let request = request_builder.body(hyper::Body::empty()).unwrap();
                let response = self.client.inner.request(request);
                self.response = Some(response);
                println!("got response");
            }

            if let Some(mut resp) = self.response.take() {
                //println!("response: poll");
                match Pin::new(&mut resp).poll(cx) {
                    Poll::Pending => {
                        //println!("response: pending");
                        self.response = Some(resp);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => {
                        println!("response: ready err {:?}", e);
                        todo!()
                        //return Poll::Ready(Some(Err(e.into())));
                    }
                    Poll::Ready(Ok(resp)) => {
                        let status = resp.status();
                        println!("response: ready ok {:?}", status);
                        println!("Resp {:?}", resp);
                        let body_stream = resp
                            .into_body()
                            .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
                            .into_async_read();

                        let decoder = Box::new(async_sse::decode(body_stream));
                        self.decoder = Some(decoder);
                    }
                }
            }

            if let Some(mut decoder) = self.decoder.take() {
                //println!("decoder: poll");
                match Pin::new(&mut decoder).poll_next(cx) {
                    Poll::Pending => {
                        //println!("decoder: pending");
                        self.decoder = Some(decoder);
                        return Poll::Pending;
                    }
                    Poll::Ready(None) => {
                        println!("decoder: none");
                        //return Poll::Ready(None);
                    }
                    Poll::Ready(Some(Err(e))) => {
                        println!("decoder: ready err {:?}", e);
                        todo!()
                        //return Poll::Ready(Some(Err(e.into())));
                    }
                    Poll::Ready(Some(Ok(message))) => {
                        self.decoder = Some(decoder);
                        match message {
                            async_sse::Event::Message(msg) => {
                                if msg.name() == "message" {
                                    let result: R::Resource =
                                        serde_json::from_slice(&msg.into_bytes()).unwrap();
                                    return Poll::Ready(Some(Ok(result)));
                                }
                            }
                            async_sse::Event::Retry(duration) => {
                                println!("got duration {:?}", duration);
                            }
                        }
                    }
                }
            }
        }
    }
}
