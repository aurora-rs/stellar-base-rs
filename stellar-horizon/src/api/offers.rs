use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use url::Url;

#[derive(Debug, Clone)]
pub struct AllOffersRequest {}

#[derive(Debug, Clone)]
pub struct SingleOfferRequest {}

#[derive(Debug, Clone)]
pub struct OffersForAccountRequest {}
