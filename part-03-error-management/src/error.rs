use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Didn't get a query string")]
    MissingQuery,
    #[error("Didn't get a file name")]
    MissingFilename,
    #[error("Could not load config, reason : {source:}")]
    ConfigLoad {
        #[from]
        source: io::Error,
    },
}
