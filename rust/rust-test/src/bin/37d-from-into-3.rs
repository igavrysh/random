use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
enum NetworkError {
    #[error("connection timed out")]
    Timeout,
}

#[allow(dead_code)]
#[derive(Debug, Error)]
enum DatabaseError {
    #[error("error querying database")]
    QueryFailure,
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] NetworkError),
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),
}

/*
impl From<NetworkError> for ApiError {
    fn from(err: NetworkError) -> Self {
        Self::Network(err)
    }
}

impl From<DatabaseError> for ApiError {
    fn from(err: DatabaseError) -> Self {
        Self::Database(err)
    }
}
*/

#[allow(dead_code)]
fn do_stuff() -> Result<(), ApiError> {
    Err(NetworkError::Timeout)?
}

fn main() {
}

