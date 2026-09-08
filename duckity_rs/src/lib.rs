//! A pure-Rust Duckity API client.
//!
//! For a more detailed documentation, check out
//! [Duckity Docs on the Rust SDK](https://duckity.com/docs/sdks/rust).
//!
//! # Installation
//!
//! To add the package to your project, install it with cargo:
//!
//! ```sh
//! $ cargo add duckity
//! ```
//!
//! # Quick Start
//!
//! First of all, you need a Duckity application. Head over to the
//! [Duckity dashboard](https://app.duckity.com) to create one if you don't have created it yet.
//!
//! To solve a challenge, use [`duckity::solve`](solve).
//!
//! ```
//! let solution = duckity::solve(application_id, protection_profile_id).await?;
//! ```
//!
//! # Compiling
//!
//! Make sure to compile in release mode when testing challenge solving. Given solving challenges
//! is completely CPU-bound, you will see massive differences between solving in debug and release
//! mode (debug was x23 slower to solve in our tests).
//!
//! # Contributing
//!
//! Contributions of any kind are welcome! Suggestions, issues, PRs, and everything else goes into
//! our [SDKs repository in GitHub](https://github.com/duckity-com/sdks).

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::String;
#[cfg(feature = "std")]
use std::net::IpAddr;
#[cfg(feature = "std")]
use std::pin::Pin;

pub use crate::error::DuckityError;
#[cfg(feature = "std")]
use crate::schemas::{ChallengeResponse, ValidateRequest, ValidateResponse};

pub mod core;
mod error;
#[cfg(feature = "std")]
mod schemas;

#[cfg(feature = "std")]
static HOSTED_BASE_URL: &str = "https://api.duckity.com/d1";

/// Gets a challenge from the Duckling API, solves it, and returns the solution.
///
/// For example:
/// ```
/// // Challenge from api.duckity.com.
/// let solution = duckity::get(protection_profile_id).await?;
///
/// // Challenge from a self-hosted duckling.
/// let solution = duckity::get(protection_profile_id)
///     .base_url("https://quack.example.com/v1")
///     .await?;
/// ```
///
/// Arguments:
/// * `protection_profile_id` - The protection profile's ID.
///
/// Returns:
/// [`ChallengeGetter`] - An awaitable builder to get a challenge.
#[cfg(feature = "std")]
pub fn solve(protection_profile_id: impl Into<String>) -> ChallengeGetter {
    ChallengeGetter {
        base_url: HOSTED_BASE_URL.to_string(),
        protection_profile_id: protection_profile_id.into(),
    }
}

/// Challenge-getting builder. Initialize with [`solve`].
#[cfg(feature = "std")]
pub struct ChallengeGetter {
    base_url: String,
    protection_profile_id: String,
}

#[cfg(feature = "std")]
impl ChallengeGetter {
    /// Sets the base URL of the duckling API.
    ///
    /// By default, this is `https://api.duckity.com/d1`. Set this to scheme + host + version base
    /// path, without trailing slash.
    ///
    /// Arguments:
    /// * `url` - The base URL.
    ///
    /// Returns:
    /// [`ChallengeGetter`] - The current builder with the base URL set.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sends the request to the duckling API to get a challenge.
    ///
    /// Returns:
    /// * `Ok(String)` - The encoded challenge string.
    /// * `Err(DuckityError)` - An error occurred while sending the request.
    pub async fn send(self) -> Result<String, DuckityError> {
        let client = reqwest::Client::new();

        let url = {
            let mut url = self.base_url;
            url.push_str(&format!("/challenges/{}/issue", self.protection_profile_id));
            url
        };

        let request = client.post(url).header("x-duckity-csrf", "1");

        let response = request.send().await?;
        let response: ChallengeResponse = response.json().await?;

        Ok(response.challenge)
    }

    /// Sends the request to the duckling API to get a challenge, solves it, and returns the
    /// solution token.
    ///
    /// This function is slow. The CPU-intensive part is spawned in a
    /// `tokio::task::spawn_blocking()` not to block the runtime.
    ///
    /// Returns:
    /// * `Ok(String)` - The encoded solution string.
    /// * `Err(DuckityError)` - An error occurred while sending the request.
    pub async fn send_and_solve(self) -> Result<String, DuckityError> {
        let challenge = self.send().await?;

        tokio::task::spawn_blocking(move || {
            let decoded = core::decode(&challenge)?;
            let solution = core::solve(&decoded);
            let encoded = core::encode(&challenge, &solution)?;

            Ok(encoded)
        })
        .await
        .unwrap()
    }
}

#[cfg(feature = "std")]
impl IntoFuture for ChallengeGetter {
    type Output = Result<String, DuckityError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.send_and_solve().await })
    }
}

#[cfg(feature = "std")]
/// Solution validation builder. Initialized via [`validate`]. Awaitable.
pub struct SolutionValidator {
    protection_profile_id: String,
    application_secret: String,
    ip: IpAddr,
    base_url: String,
    solution: String,
}

#[cfg(feature = "std")]
impl SolutionValidator {
    /// Sets the base URL of the duckling API.
    ///
    /// By default, this is `https://api.duckity.com/d1`. Set this to scheme + host + version base
    /// path, without trailing slash.
    ///
    /// Arguments:
    /// * `url` - The base URL.
    ///
    /// Returns:
    /// [`ChallengeGetter`] - The current builder with the base URL set.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sends the request to the duckling API to validate a challenge solution.
    ///
    /// Returns:
    /// * `Ok(bool)` - Whether the challenge solution was valid.
    /// * `Err(DuckityError)` - An error occurred while sending the request.
    async fn send(self) -> Result<bool, DuckityError> {
        let client = reqwest::Client::new();

        let url = {
            let mut url = self.base_url;
            url.push_str(&format!(
                "/challenges/{}/validate",
                self.protection_profile_id
            ));
            url
        };

        let request = client
            .post(url)
            .json(&ValidateRequest {
                solution: self.solution,
                ip: self.ip,
            })
            .bearer_auth(self.application_secret);

        let response = request.send().await?;
        let response: ValidateResponse = response.json().await?;

        Ok(response.is_valid)
    }
}

#[cfg(feature = "std")]
impl IntoFuture for SolutionValidator {
    type Output = Result<bool, DuckityError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.send().await })
    }
}

#[cfg(feature = "std")]
/// Validates a solution token.
///
/// Arguments:
/// * `solution` - The solution token to validate.
/// * `ip` - The IP of the client that submitted the solution.
/// * `application_secret` - The secret of the protection profile's application.
/// * `protection_profile_id` - The ID of the protection profile this challenge was submitted for.
pub fn validate(
    solution: impl Into<String>,
    ip: impl Into<IpAddr>,
    application_secret: impl Into<String>,
    protection_profile_id: impl Into<String>,
) -> SolutionValidator {
    SolutionValidator {
        protection_profile_id: protection_profile_id.into(),
        application_secret: application_secret.into(),
        ip: ip.into(),
        base_url: HOSTED_BASE_URL.to_string(),
        solution: solution.into(),
    }
}
