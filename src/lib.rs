//! # base_x
//!
//! Encode and decode any base alphabet.
//!
//! ## Installation
//!
//! Add this to `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! base-x = "0.2.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! extern crate base_x;
//!
//! fn main() {
//!   let decoded = base_x::decode("01", "11111111000000001111111100000000").unwrap();
//!   let encoded = base_x::encode("01", &decoded);
//!  assert_eq!(encoded, "11111111000000001111111100000000");
//! }
//! ```

mod bigint;
pub mod decoder;
pub mod encoder;
pub mod alphabet;

pub use decoder::{AsciiDecoder, Utf8Decoder};
pub use encoder::{AsciiEncoder, Utf8Encoder};
pub use alphabet::Alphabet;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to decode the given data")
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        "Can not decode the provided data"
    }
}

/// Encode an input vector using the given alphabet.
pub fn encode<A: Alphabet>(alphabet: A, input: &[u8]) -> String {
    alphabet.encode(input)
}

/// Decode an input vector using the given alphabet.
pub fn decode<A: Alphabet>(alphabet: A, input: &str) -> Result<Vec<u8>, DecodeError> {
    alphabet.decode(input)
}

#[cfg(test)]
mod test {
    use super::encode;
    use super::decode;
    extern crate json;
    use self::json::parse;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn works() {
        let mut file = File::open("./fixtures/fixtures.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = parse(&data).unwrap();
        let alphabets = &json["alphabets"];

        for value in json["valid"].members() {
            let alphabet_name = value["alphabet"].as_str().unwrap();
            let input = value["string"].as_str().unwrap();
            let alphabet = alphabets[alphabet_name].as_str().unwrap();

            // Alphabet works as unicode
            let decoded = decode(alphabet, input).unwrap();
            let encoded = encode(alphabet, &decoded);
            assert_eq!(encoded, input);

            // Alphabet works as ASCII
            let decoded = decode(alphabet.as_bytes(), input).unwrap();
            let encoded = encode(alphabet.as_bytes(), &decoded);
            assert_eq!(encoded, input);
        }
    }

    #[test]
    fn is_unicode_sound() {
        // binary, kinda...
        let alphabet = "😐😀";

        let encoded = encode(alphabet, &[0xff,0x00,0xff,0x00]);
        let decoded = decode(alphabet, &encoded).unwrap();

        assert_eq!(encoded, "😀😀😀😀😀😀😀😀😐😐😐😐😐😐😐😐😀😀😀😀😀😀😀😀😐😐😐😐😐😐😐😐");
        assert_eq!(decoded, &[0xff,0x00,0xff,0x00]);
    }
}
