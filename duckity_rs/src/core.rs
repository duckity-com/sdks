//! Low-level implementation of challenge decoding, solving, and encoding.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::net::IpAddr;

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
#[derive(Clone, Serialize, Deserialize)]
pub struct Challenge {
    /// The IP of the client this challenge was issued for.
    #[cfg(feature = "std")]
    pub ip: IpAddr,
    /// The IP of the client this challenge was issued for.
    #[cfg(not(feature = "std"))]
    pub ip: String,
    /// This challenge's unique ID.
    #[serde(alias = "challenge_id")]
    pub id: String,
    n: Vec<u32>,
    x: Vec<u32>,
    t: u32,
}

impl Challenge {
    /// Returns the challenge's hardness.
    ///
    /// This is a reader for `Challenge::t`.
    ///
    /// Returns:
    /// [`u32`] - The challenge's hardness.
    pub fn hardness(&self) -> u32 {
        self.t
    }
}

/// The solution to a challenge.
#[derive(Clone, Serialize, Deserialize)]
pub struct Solution {
    y: Vec<u32>,
    pi: Vec<u32>,
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

fn get_digits_from_integer_buf<T>(integer: impl Into<Integer>, buffer: &mut [T])
where
    T: rug::integer::UnsignedPrimitive,
{
    let integer = integer.into();
    let slice_start = buffer
        .len()
        .saturating_sub(integer.significant_digits::<T>());

    integer.write_digits(&mut buffer[slice_start..], Order::Msf);
}

fn get_digits_from_integer<I, T>(integer: I, array_width: usize, zero: T) -> Vec<T>
where
    T: rug::integer::UnsignedPrimitive + Copy,
    I: Into<Integer>,
{
    let mut buffer = vec![zero; array_width];

    get_digits_from_integer_buf(integer, &mut buffer);

    buffer
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

    let mut bytes: Vec<u8> = b"duckity".to_vec();
    bytes.append(&mut get_digits_from_integer(n.clone(), 512, 0));
    bytes.append(&mut get_digits_from_integer(x.clone(), 512, 0));
    bytes.append(&mut get_digits_from_integer(challenge.t, 512, 0));
    bytes.append(&mut get_digits_from_integer(y.clone(), 512, 0));

    let z = Sha256::digest(bytes);
    let z_int = Integer::from_digits(&z, Order::Msf);
    let l = z_int.next_prime();

    let mut r = Integer::from(1);
    let mut s = Integer::from(1);

    for _ in 0..challenge.t {
        r *= 2;

        if r >= l {
            r -= &l;
            s = ((&s * &s).complete() * &x) % &n;
        } else {
            s = (&s * &s).complete() % &n;
        }
    }

    Solution {
        y: get_digits_from_integer(y, 128, 0),
        pi: get_digits_from_integer(s, 128, 0),
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
