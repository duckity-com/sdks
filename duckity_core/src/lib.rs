//! A no-std implementation of Duckity challenge solving.
//!
//! The no-std implementation may be slower than the std implementation, therefore if you can use
//! std in your environment, use [duckity-rs](https://docs.rs/duckity) instead of duckity-core.
//!
//! This crate only contains primitives to decode, solve, and encode challenges. You'll have to
//! implement the HTTP API part yourself.
//!
//! # Installation
//!
//! To install this package, run the following cargo command:
//!
//! ```sh
//! $ cargo add duckity-core
//! ```
//!
//! # Usage
//!
//! To begin with, fetch a challenge from the duckling API. You do so with a request that looks
//! like follows:
//!
//! ```http
//! GET /d1/challenges/{protection_profile_id}/issue HTTP/1.1
//! Host: api.duckity.com
//! ```
//!
//! If successful, you'll get a response that looks like follows:
//!
//! ```http
//! HTTP/1.1 200 OK
//! Content-Type: application/json
//!
//! {
//!   "challenge": "challenge-string-here"
//! }
//! ```
//!
//! > *Note: Read the Duckity documentation as this is not strictly kept up-to-date.*
//!
//! Once you have the challenge string, you have to decode it, solve it, and encode the solution.
//!
//! ```
//! let challenge: String;
//!
//! let challenge_decoded = duckity_core::decode(&challenge)?;
//! let solution = duckity_core::solve(&challenge_decoded)?; // Slow and CPU-intensive!
//! let solution_encoded = duckity_core::encode(&challenge, &solution)?;
//! ```
//!
//! Your `solution_encoded` variable is the token you'll send back to your backend server.
//!
//! # Compiling
//!
//! Make sure to compile in release mode when testing challenge solving. Given solving challenges
//! is completely CPU-bound, you will see massive differences between solving in debug and release
//! mode.
//!
//! # Contributing
//!
//! Contributions of any kind are welcome! Suggestions, issues, PRs, and everything else goes into
//! our [SDKs repository in GitHub](https://github.com/duckity-dev/sdks). We reward good
//! contributions with Duckity Pro tiers 😉

#![no_std]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use num_bigint_dig::{BigInt, Sign, ToBigInt, prime};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// The meaningful-to-the-client parts of a challenge.
///
/// This cannot be re-encoded into the original challenge string. Keep both.
#[derive(Serialize, Deserialize)]
pub struct Challenge {
    n: Vec<u32>,
    x: Vec<u32>,
    t: u32,
}

/// The solution to a challenge.
#[derive(Serialize, Deserialize)]
pub struct Solution {
    pub y: Vec<u8>,
    pub pi: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum DuckityDecodeError {
    #[error("The challenge string passed to decode() did not have enough parts.")]
    NotEnoughParts,

    #[error(
        "The challenge string passed to decode() had its first section not being url-safe base64."
    )]
    NotBase64(base64::DecodeError),

    #[error("The challenge string passed to decode() had an invalid challenge section.")]
    NotAChallenge(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum DuckityEncodeError {
    #[error("The challenge's solution could not be encoded into JSON.")]
    Json(#[from] serde_json::Error),
}

/// Decodes a challenge string into a [`Challenge`].
///
/// Arguments:
/// * `challenge` - The raw challenge string returned by the API.
///
/// Returns:
/// * `Ok(Challenge)` - The decoded challenge.
/// * `Err(DuckityError)` - The challenge string could not be decoded.
pub fn decode(challenge: &str) -> Result<Challenge, DuckityDecodeError> {
    let (challenge, _signature) = challenge
        .split_once(".")
        .ok_or(DuckityDecodeError::NotEnoughParts)?;

    let bytes = BASE64_URL_SAFE_NO_PAD
        .decode(challenge)
        .map_err(DuckityDecodeError::NotBase64)?;

    let decoded: Challenge = serde_json::from_slice(&bytes)?;

    Ok(decoded)
}

/// Solves a challenge.
///
/// This function is meant to be slow and CPU-intensive. Do not run it on the UI thread.
///
/// Arguments:
/// * `challenge` - The challenge to solve.
///
/// Returns:
/// [`Solution`] - The solution to the challenge.
pub fn solve(challenge: &Challenge) -> Solution {
    let n = BigInt::from_slice(Sign::Plus, &challenge.n);
    let x = BigInt::from_slice(Sign::Plus, &challenge.x);

    let mut y = x.clone();

    for _ in 0..challenge.t {
        y = y.modpow(&BigInt::from(2), &n);
    }

    let mut bytes = x.to_bytes_be().1;
    bytes.append(&mut y.to_bytes_be().1);

    let z = Sha256::digest(bytes);
    let z_int = BigInt::from_bytes_be(Sign::Plus, &z);
    let l = prime::next_prime(&z_int.to_biguint().unwrap())
        .to_bigint()
        .unwrap();

    let mut pi = BigInt::from(1);
    let mut acc = x.clone();
    let mut exp_mod_l = BigInt::from(1);

    for _ in 0..challenge.t {
        let doubled = &exp_mod_l * 2u32;
        if doubled >= l {
            pi = (&pi * &acc) % &n;
            exp_mod_l = &doubled - &l;
        } else {
            exp_mod_l = doubled;
        }

        acc = acc.modpow(&BigInt::from(2), &n);
    }

    Solution {
        y: y.to_bytes_be().1,
        pi: pi.to_bytes_be().1,
    }
}

/// Encodes a challenge's solution to a solved challenge token.
///
/// Arguments:
/// * `original` - The original challenge string, raw.
/// * `solution` - The solution to the challenge.
///
/// Returns:
/// * `Ok(String)` - The encoded solved challenge token.
/// * `Err(DuckityEncodeError)` - The solution could not be encoded into JSON.
pub fn encode(original: &str, solution: &Solution) -> Result<String, DuckityEncodeError> {
    let solution_json = serde_json::to_vec(solution)?;
    let mut solution_base64 = BASE64_URL_SAFE_NO_PAD.encode(&solution_json);

    solution_base64.insert(0, '.');
    solution_base64.insert_str(0, original);

    Ok(solution_base64)
}
