use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use url::Url;

#[derive(Debug, Clone)]
pub struct AllPaymentsRequest {}

#[derive(Debug, Clone)]
pub struct PaymentsForAccountRequest {}
