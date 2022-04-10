//! Errors

/// Crate error
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("TOML serialization error: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Crate result type
pub type Result<T> = std::result::Result<T, Error>;
