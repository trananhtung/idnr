//! # idnr — validate the German tax identification number
//!
//! Validate a German *Steuerliche Identifikationsnummer* (IdNr / Steuer-IdNr) — the 11-digit
//! personal tax number assigned to individuals in Germany. A faithful Rust port of the
//! algorithm used by [`python-stdnum`](https://arthurdejong.org/python-stdnum/).
//!
//! ```
//! use idnr::{is_valid, validate};
//!
//! assert!(is_valid("36574261809"));
//! assert!(is_valid("36 574 261 809"));   // separators are accepted
//! assert!(!is_valid("36574261890"));     // wrong check digit
//! assert!(!is_valid("36554266806"));     // too many repeated digits
//! ```
//!
//! Validation checks the length (11 digits), that the number does not start with `0`, the
//! digit-repetition rule (exactly one digit in the first ten appears two or three times), and
//! the ISO 7064 MOD 11,10 check digit.
//!
//! **Zero dependencies** and `#![no_std]`.

#![no_std]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/idnr/0.1.0")]
// German proper nouns (IdNr, Steuer-IdNr, …) in the docs are prose, not code.
#![allow(clippy::doc_markdown)]

extern crate alloc;

use alloc::string::{String, ToString};
use core::fmt;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Why a number is not a valid IdNr.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The number does not have exactly 11 digits.
    InvalidLength,
    /// The number is malformed (non-digit, leading zero, or wrong digit repetition).
    InvalidFormat,
    /// The check digit does not match.
    InvalidChecksum,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Error::InvalidLength => "number must be 11 digits long",
            Error::InvalidFormat => "number is malformed",
            Error::InvalidChecksum => "check digit does not match",
        };
        f.write_str(message)
    }
}

impl core::error::Error for Error {}

/// Strip separators (`-`, `.`, `/`, `,`, space) and surrounding whitespace.
#[must_use]
pub fn compact(number: &str) -> String {
    number
        .chars()
        .filter(|c| !matches!(c, ' ' | '-' | '.' | '/' | ','))
        .collect::<String>()
        .trim()
        .to_string()
}

/// The ISO 7064 MOD 11,10 running checksum (a valid number has a checksum of `1`).
fn mod_11_10_checksum(digits: &[u8]) -> i32 {
    let mut check = 5i32;
    for &digit in digits {
        let carry = if check == 0 { 10 } else { check };
        check = ((carry * 2) % 11 + i32::from(digit)) % 10;
    }
    check
}

/// Calculate the IdNr check digit (the 11th digit) for the first ten digits of a number.
///
/// # Errors
/// Returns [`Error::InvalidFormat`] if `first_ten` is not exactly ten digits.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // `check` is `0..=9`.
pub fn calc_check_digit(first_ten: &str) -> Result<char, Error> {
    let digits = parse_digits(first_ten).ok_or(Error::InvalidFormat)?;
    if digits.len() != 10 {
        return Err(Error::InvalidFormat);
    }
    let checksum = mod_11_10_checksum(&digits);
    let carry = if checksum == 0 { 10 } else { checksum };
    let check = (1 - (carry * 2) % 11).rem_euclid(10);
    Ok(char::from(b'0' + check as u8))
}

fn parse_digits(value: &str) -> Option<alloc::vec::Vec<u8>> {
    value
        .bytes()
        .map(|b| b.is_ascii_digit().then_some(b - b'0'))
        .collect()
}

/// Validate a German IdNr, returning the compacted number on success.
///
/// # Errors
/// Returns [`Error::InvalidLength`], [`Error::InvalidFormat`], or [`Error::InvalidChecksum`].
pub fn validate(number: &str) -> Result<String, Error> {
    let number = compact(number);
    let bytes = number.as_bytes();

    if bytes.len() != 11 {
        return Err(Error::InvalidLength);
    }
    if !bytes.iter().all(u8::is_ascii_digit) {
        return Err(Error::InvalidFormat);
    }
    if bytes[0] == b'0' {
        return Err(Error::InvalidFormat);
    }

    // In the first ten digits, exactly one digit appears two or three times and every other
    // digit appears at most once.
    let mut counts = [0u8; 10];
    for &byte in &bytes[..10] {
        counts[usize::from(byte - b'0')] += 1;
    }
    let repeated: alloc::vec::Vec<u8> = counts.iter().copied().filter(|&c| c > 1).collect();
    if repeated.len() != 1 || !matches!(repeated[0], 2 | 3) {
        return Err(Error::InvalidFormat);
    }

    let digits: alloc::vec::Vec<u8> = bytes.iter().map(|&b| b - b'0').collect();
    if mod_11_10_checksum(&digits) != 1 {
        return Err(Error::InvalidChecksum);
    }
    Ok(number)
}

/// Whether `number` is a valid German IdNr.
///
/// ```
/// # use idnr::is_valid;
/// assert!(is_valid("86095742719"));
/// assert!(!is_valid("01234567890")); // must not start with 0
/// ```
#[must_use]
pub fn is_valid(number: &str) -> bool {
    validate(number).is_ok()
}

/// Reformat a number to the standard `NN NNN NNN NNN` presentation.
///
/// # Errors
/// Returns an error if `number` is not a valid IdNr.
pub fn format(number: &str) -> Result<String, Error> {
    let number = validate(number)?;
    let b = number.as_bytes();
    Ok(alloc::format!(
        "{} {} {} {}",
        &number[0..2],
        core::str::from_utf8(&b[2..5]).unwrap_or_default(),
        &number[5..8],
        &number[8..11],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_numbers() {
        assert!(is_valid("36574261809"));
        assert!(is_valid("86095742719"));
        assert_eq!(validate("36 574 261 809").unwrap(), "36574261809");
        assert_eq!(validate("36-574-261-809").unwrap(), "36574261809");
    }

    #[test]
    fn invalid_numbers() {
        assert_eq!(validate("36574261890"), Err(Error::InvalidChecksum));
        assert_eq!(validate("36554266806"), Err(Error::InvalidFormat)); // too many repeats
        assert_eq!(validate("01234567890"), Err(Error::InvalidFormat)); // leading zero
        assert_eq!(validate("3657426180"), Err(Error::InvalidLength)); // 10 digits
        assert_eq!(validate("3657426180a"), Err(Error::InvalidFormat)); // non-digit
        assert!(!is_valid(""));
    }

    #[test]
    fn check_digit() {
        assert_eq!(calc_check_digit("0123456789").unwrap(), '6');
        assert_eq!(calc_check_digit("3657426180").unwrap(), '9'); // 36574261809
        assert_eq!(calc_check_digit("123").unwrap_err(), Error::InvalidFormat); // wrong length
    }

    #[test]
    fn formatting() {
        assert_eq!(format("36574261809").unwrap(), "36 574 261 809");
        assert!(format("00000000000").is_err());
    }
}
