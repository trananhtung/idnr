# idnr

[![crates.io](https://img.shields.io/crates/v/idnr.svg)](https://crates.io/crates/idnr)
[![docs.rs](https://docs.rs/idnr/badge.svg)](https://docs.rs/idnr)
[![CI](https://github.com/trananhtung/idnr/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/idnr/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/idnr.svg)](#license)

**Validate the German tax identification number (Steuerliche Identifikationsnummer / IdNr).**

The IdNr (Steuer-IdNr) is the 11-digit personal tax number assigned to individuals in Germany.
A faithful Rust port of the algorithm used by
[`python-stdnum`](https://arthurdejong.org/python-stdnum/).

- **Zero dependencies**, **`#![no_std]`**
- `is_valid`, `validate`, `calc_check_digit`, `compact`, `format`
- Validates the length, leading-zero rule, the digit-repetition rule, and the ISO 7064
  MOD 11,10 check digit
- Differential-tested against `python-stdnum` (60k cases)

## Install

```toml
[dependencies]
idnr = "0.1"
```

## Usage

```rust
use idnr::{is_valid, validate, format};

assert!(is_valid("36574261809"));
assert!(is_valid("36 574 261 809"));     // separators accepted
assert!(!is_valid("36574261890"));       // wrong check digit
assert!(!is_valid("36554266806"));       // too many repeated digits

assert_eq!(validate("36.574.261.809").unwrap(), "36574261809");
assert_eq!(format("36574261809").unwrap(), "36 574 261 809");
```

A valid IdNr has 11 digits, does not start with `0`, has exactly one digit among its first ten
appearing two or three times (others at most once), and passes the ISO 7064 MOD 11,10 check.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
