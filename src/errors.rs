use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Error happened on database connection")]
    Connection,

    #[error("Error happened on database query: {0}")]
    QueryError(String),

    #[error("Error happened on database insert: {0}")]
    InsertError(String),

    #[error("Error happened on database update: {0}")]
    UpdateError(String),

    #[error("Error happened on database delete: {0}")]
    DeleteError(String),
}
