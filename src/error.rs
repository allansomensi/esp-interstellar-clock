use esp_idf_svc::{io::EspIOError, sys::EspError};

/// Represents errors that can occur in the application.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    IO(#[from] EspIOError),

    #[error("System error: {0}")]
    System(#[from] EspError),

    #[error("Display error: {0}")]
    Display(String),
}

impl From<tm1637::Error<EspError>> for AppError {
    fn from(value: tm1637::Error<EspError>) -> Self {
        AppError::Display(format!("{:?}", value))
    }
}
