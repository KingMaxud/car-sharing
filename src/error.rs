use std::fmt::Formatter;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum CarSharingError {
    DatabaseDieselError(diesel::result::Error),
    DatabaseIntParsingError(ParseIntError),
    DatabaseNotFound,
}

pub type Result<T> = std::result::Result<T, CarSharingError>;

impl std::fmt::Display for CarSharingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            _ => write!(f, "Error"),
        }
    }
}

impl From<diesel::result::Error> for CarSharingError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => CarSharingError::DatabaseNotFound,
            _ => CarSharingError::DatabaseDieselError(err),
        }
    }
}

impl From<ParseIntError> for CarSharingError {
    fn from(value: ParseIntError) -> Self {
        CarSharingError::DatabaseIntParsingError(value)
    }
}
