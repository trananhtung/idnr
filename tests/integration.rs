//! Integration tests exercising the public API of `idnr`.

use idnr::{calc_check_digit, format, is_valid, validate, Error};

#[test]
fn accepts_valid() {
    for n in ["36574261809", "86095742719", "47036892816"] {
        assert!(is_valid(n), "{n} should be valid");
    }
}

#[test]
fn separators_accepted() {
    assert_eq!(validate("36 574 261 809").unwrap(), "36574261809");
    assert_eq!(validate("36.574.261.809").unwrap(), "36574261809");
    assert_eq!(validate("36/574/261/809").unwrap(), "36574261809");
}

#[test]
fn rejects_invalid() {
    assert_eq!(validate("36574261890"), Err(Error::InvalidChecksum));
    assert_eq!(validate("36554266806"), Err(Error::InvalidFormat)); // repeated digits
    assert_eq!(validate("00000000000"), Err(Error::InvalidFormat)); // leading zero
    assert_eq!(validate("123"), Err(Error::InvalidLength));
}

#[test]
fn check_digit_and_format() {
    assert_eq!(calc_check_digit("3657426180").unwrap(), '9');
    assert_eq!(format("86095742719").unwrap(), "86 095 742 719");
}
