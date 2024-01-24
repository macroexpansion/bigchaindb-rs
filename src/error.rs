use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request timeout")]
    RequestTimeout,

    #[error("no JSON body data provided for the POST request")]
    RequestNoBodyProvided,

    #[error("request error")]
    RequestError,

    #[error("internal error")]
    InternalError,

    #[error("HTTP Error: Requested page not reachable")]
    ResponseError,

    #[error("serde_json error")]
    SerdeError,
}
