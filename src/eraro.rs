use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Eraro {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::error::ImageError),
    #[error("Bincode encoding error: {0}")]
    BincodeEncoding(#[from] bincode::error::EncodeError),
    #[error("Bincode decoding error: {0}")]
    BincodeDecoding(#[from] bincode::error::DecodeError),
}
