use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("language {0} not found")]
    LanguageNotFound(String),

    #[error(
        "TIO API error {status}{reason}",
        reason = .reason.map_or_else(String::new, |s| format!(": {s}")),
    )]
    ApiError {
        resp: reqwest::Response,
        status: u16,
        reason: Option<&'static str>,
    },

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<reqwest::Response> for Error {
    fn from(resp: reqwest::Response) -> Self {
        let code = resp.status();
        Self::ApiError {
            resp,
            status: code.as_u16(),
            reason: code.canonical_reason(),
        }
    }
}
