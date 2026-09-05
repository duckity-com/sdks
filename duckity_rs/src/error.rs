//! A wrapper over all errors returned by this crate.

use thiserror::Error;

use crate::core;

/// Wrapper over all possible errors returned by the `duckity` crate.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum DuckityError {
    #[cfg(feature = "std")]
    #[error("An error occurred when making a request to Duckity's API: {0}")]
    Request(#[from] reqwest::Error),

    Decoding(#[from] core::DuckityDecodeError),
    Encoding(#[from] core::DuckityEncodeError),
}
