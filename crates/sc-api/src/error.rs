use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Cannot found client id in JS bunle")]
    ClientIDNotFOund,
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
