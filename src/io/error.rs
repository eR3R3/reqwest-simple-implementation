use thiserror::Error;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Invalid timeout: timeout must be greater than 10 seconds")]
    InvalidTimeout,

}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Invalid Http Version: {0}")]
    InvalidHttpVersion
}


pub enum ErrorKind {
    Request(RequestError),
    Client(ClientError),
}
