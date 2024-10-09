use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Error happened on database connection")]
    Connection,

    #[error("Error happened on database query: {0}")]
    QueryError(String),

    #[error("Error happened on database insert: {0}")]
    InsertError(String),
}
