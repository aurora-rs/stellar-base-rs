pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("stellar base error")]
    StellarBaseError(#[from] stellar_base::error::Error),
    #[error("http error")]
    HttpError(#[from] http::Error),
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    #[error("json error")]
    JsonError(#[from] serde_json::error::Error),
    #[error("invalid uri")]
    InvalidUri(#[from] http::uri::InvalidUri),
}
