// Topic: TryFrom/TryInto
//
// Summary:
// * A library is needed for an application to convert hex color codes
//   into their component color values (red, green, and blue). Hex color codes
//   consist of a hash symbol followed by six hex digits. Every two hex digits
//   represent a color component in the order of red, green, blue.
//
//   Example hex color codes:
//    #ffffff -> Rgb(255, 255, 255)
//    #001122 -> Rgb(0, 17, 34)
//
// Requirements:
// * Create a program to convert a hex code (as &str) into an Rgb struct
// * Implement TryFrom to perform the conversion
// * Utilize the question mark operator in your implementation
//
// Notes:
// * See the `from_str_radix` function in the stdlib docs for `u8`
//   to convert hex digits to `u8`
//   * Hex digits use a radix value of 16
// * Utilize the `thiserror` crate for your error type
// * Run `cargo test --bin a37` to test your implementation
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

#[derive(Debug, Error)]
enum RgbConvertError {
    #[error("parse string error: {0}")]
    ParseStrError(#[from] ParseIntError),
    #[error("out of range error, should be in range of [0, 255], actual is {0}")]
    OutOfRangeError(u32),
    #[error("wrong format for color, must be in range [\"#000000\"-\"#FFFFFF\"], actual string: {0}")]
    WrongFormat(String),
}

fn hex_chars_to_rgb_comp(h: &char, l: &char) -> Result<u8, RgbConvertError> {
    let digit = u32::from_str_radix(&format!("{}{}", h, l), 16)?;
    match digit {
        val @ 0..=255 => Ok(val as u8),
        val => Err(RgbConvertError::OutOfRangeError(val))
    }
}

impl TryFrom<&str> for Rgb {
    type Error = RgbConvertError;

    fn try_from(value: &str) -> Result<Self, RgbConvertError> {
        let chrs: Vec<_> = value.chars().collect();
        match chrs.as_slice() {
            [_first @ '#', r_h, r_l, g_h, g_l, b_h, b_l] => {
                let r = hex_chars_to_rgb_comp(r_h, r_l)?;
                let g = hex_chars_to_rgb_comp(g_h, g_l)?;
                let b = hex_chars_to_rgb_comp(b_h, b_l)?;
                Ok(Rgb(r, g, b))
            }
            [..] => Err(RgbConvertError::WrongFormat(value.to_owned()))
        }
    }
}

fn main() {
    // Use `cargo test --bin a37` to test your implementation
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(0, 204, 102);
        let actual = Rgb::try_from("#00cc66");
        assert_eq!(
            actual.is_ok(),
            true,
            "valid hex code should be converted to Rgb"
        );
        assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert_eq!(
            Rgb::try_from("#0011yy").is_err(),
            true,
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert_eq!(
            Rgb::try_from("001100").is_err(),
            true,
            "should be an error when missing hash symbol"
        );
    }

    #[test]
    fn fails_when_missing_color_components() {
        assert_eq!(
            Rgb::try_from("#0011f").is_err(),
            true,
            "should be an error when missing one or more color components"
        );
    }

    #[test]
    fn fails_with_too_many_color_components() {
        assert_eq!(
            Rgb::try_from("#0011ffa").is_err(),
            true,
            "should be an error when too many color components are provided"
        );
    }
}
