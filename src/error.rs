use std::fmt::Formatter;

use deadpool_diesel::{InteractError, PoolError};

#[derive(Debug)]
pub enum CarSharingError {
    DatabasePoolError(PoolError),
    DatabaseInteractionError(InteractError),
    DatabaseDieselError(diesel::result::Error),
    DatabaseNotFound,
}

pub type Result<T> = std::result::Result<T, CarSharingError>;

impl std::fmt::Display for CarSharingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::DatabasePoolError(err) => err.fmt(f),
            _ => write!(f, "Error"),
        }
    }
}

impl From<PoolError> for CarSharingError {
    fn from(value: PoolError) -> Self {
        CarSharingError::DatabasePoolError(value)
    }
}

impl From<InteractError> for CarSharingError {
    fn from(value: InteractError) -> Self {
        CarSharingError::DatabaseInteractionError(value)
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

// use std::fmt;
//
// use deadpool_diesel::{InteractError, PoolError};
//
// #[derive(Debug)]
// pub enum InfraError {
//     InternalServerError,
// }
//
// // Utility function to adapt errors of generic type T into InfraError
// pub fn adapt_infra_error<T: Error>(error: T) -> InfraError {
//     error.as_infra_error()
// }
//
// // Implement the Display trait to customize how InfraError is displayed
// impl fmt::Display for InfraError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             InfraError::InternalServerError => write!(f, "Internal server error"),
//         }
//     }
// }
//
// // Define a custom Error trait for types that can be converted to InfraError
// pub trait Error {
//     fn as_infra_error(&self) -> InfraError;
// }
//
// impl Error for PoolError {
//     fn as_infra_error(&self) -> InfraError {
//         InfraError::InternalServerError
//     }
// }
//
// // Implement the Error trait for InteractError
// impl Error for InteractError {
//     fn as_infra_error(&self) -> InfraError {
//         InfraError::InternalServerError      // Map all InteractError instances to InfraError::InternalServerError
//     }
// }
