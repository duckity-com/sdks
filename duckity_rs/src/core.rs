//! Low-level implementation of challenge decoding, solving, and encoding.

use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use rug::integer::Order;
use rug::{Complete, Integer};
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
    pub y: Vec<u32>,
    pub pi: Vec<u32>,
}

/// An error occurred while decoding a challenge string.
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

/// An error occurred while encoding a challenge solution into a string.
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
    let n = Integer::from_digits(&challenge.n, Order::Msf);
    let x = Integer::from_digits(&challenge.x, Order::Msf);

    let mut y = x.clone();

    for _ in 0..challenge.t {
        y = y.pow_mod(&Integer::from(2), &n).unwrap();
    }

    let mut bytes: Vec<u8> = x.to_digits(Order::Msf);
    bytes.append(&mut y.to_digits(Order::Msf));

    let z = Sha256::digest(bytes);
    let z_int = Integer::from_digits(&z, Order::Msf);
    let l = z_int.next_prime();

    let mut pi = Integer::from(1);
    let mut acc = x.clone();
    let mut exp_mod_l = Integer::from(1);

    for _ in 0..challenge.t {
        let doubled = (&exp_mod_l * 2u32).complete();
        if doubled >= l {
            pi = (&pi * &acc).complete() % &n;
            exp_mod_l = (&doubled - &l).complete();
        } else {
            exp_mod_l = doubled;
        }

        acc = acc.pow_mod(&Integer::from(2), &n).unwrap();
    }

    Solution {
        y: y.to_digits(Order::Msf),
        pi: pi.to_digits(Order::Msf),
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
