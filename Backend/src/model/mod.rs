use thiserror::Error as ThisError;
mod db;
mod todo;

// here we are writing are own error since currently we are using the package specific error
// it is better to have own custom errors
#[derive(ThisError, Debug)]
pub enum Error {
    // generally we have to
    #[error("Entity not found {0} ---- {1}")]
    EntityNotFound(&'static str, String), // custom error that shows entity name and the string -> id

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
