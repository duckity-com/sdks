//! A pure-Rust Duckity API client.
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
//! To get a challenge, use [`duckity::get`](get).
//!
//! ```
//! let challenge = duckity::get(application_id, protection_profile_id).await?;
//! ```
//!
//! If you self-host your duckling, use [`duckity::get().base_url()`](ChallengeGetter::base_url) to
//! point the client to your duckling like follows:
//!
//! ```
//! let challenge = duckity::get(application_id, protection_profile_id)
//!     .base_url("https://quack.example.com/v1")
//!     .await?;
//! ```
//!
//! Once you have a challenge string, solve it with [`duckity::solve`](solve).
//!
//! ```
//! let solution = duckity::solve(challenge)?;
//! ```
//!
//! Note that solving a challenge is CPU-intensive. Use a separate thread if you do not want to
//! freeze your application's execution. Do not run directly in async contexts either, check your
//! runtime's documentation on how to run blocking code.
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
//! our [SDKs repository in GitHub](https://github.com/duckity-dev/sdks). We reward good
//! contributions with Duckity Pro tiers 😉

use std::pin::Pin;

pub use crate::error::DuckityError;
use crate::schemas::ChallengeResponse;

pub mod core;
mod error;
mod schemas;

/// Get a challenge from the Duckling API.
///
/// For example:
/// ```
/// // Challenge from api.duckity.com.
/// let challenge = duckity::get(protection_profile_id).await?;
///
/// // Challenge from a self-hosted duckling.
/// let challenge = duckity::get(protection_profile_id)
///     .base_url("https://quack.example.com/v1")
///     .await?;
/// ```
///
/// Arguments:
/// * `protection_profile_id` - The protection profile's ID.
///
/// Returns:
/// [`ChallengeGetter`] - An awaitable builder to get a challenge.
pub fn get(protection_profile_id: impl Into<String>) -> ChallengeGetter {
    ChallengeGetter {
        base_url: "https://api.duckity.com/d1".into(),
        protection_profile_id: protection_profile_id.into(),
    }
}

/// Challenge-getting builder. Initialize with [`get`].
pub struct ChallengeGetter {
    base_url: String,
    protection_profile_id: String,
}

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
    async fn send(self) -> Result<String, DuckityError> {
        let client = reqwest::Client::new();

        let url = {
            let mut url = self.base_url;
            url.push_str(&format!("/challenges/{}/issue", self.protection_profile_id));
            url
        };

        let request = client.post(url);

        let response = request.send().await?;
        let response: ChallengeResponse = response.json().await?;

        Ok(response.challenge)
    }
}

impl IntoFuture for ChallengeGetter {
    type Output = Result<String, DuckityError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.send().await })
    }
}

/// Solves a challenge string and returns an encoded solution string.
///
/// This function is slow and CPU-intensive, do not call it from an async context.
///
/// Arguments:
/// * `challenge` - The challenge string to solve.
///
/// Returns:
/// * `Ok(String)` - The encoded solution string.
/// * `Err(DuckityError)` - An error occurred while decoding the challenge or encoding the
///   solution.
pub fn solve(challenge: &str) -> Result<String, DuckityError> {
    let decoded_challenge = core::decode(challenge)?;
    let solution = core::solve(&decoded_challenge);
    let encoded_solution = core::encode(challenge, &solution)?;

    Ok(encoded_solution)
}
