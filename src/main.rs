use std::fmt::Display;
use std::str::FromStr;
use core::str;

#[derive(Debug, PartialEq)]
struct Rgb {
    red: u8,
    blue: u8,
    green: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.red
    }

    fn g(&self) -> u8 {
        self.green
    }

    fn b(&self) -> u8 {
        self.blue
    }
}

#[derive(Debug)]
enum ParseColorError {
    RedChannelOutOfBounds,
    GreenChannelOutOfBounds,
    BlueChannelOutOfBounds,
    NoLeadingHash,
}

impl FromStr for Rgb {

    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // their solution. Gets the error involved.
        if let Some(hex_only) = s.strip_prefix('#') {
            let r = u8::from_str_radix(&hex_only[0..2], 16)
                                          .or_else(|_| Err(ParseColorError::RedChannelOutOfBounds))?;
            let g = u8::from_str_radix(&hex_only[2..4], 16)
                                        .or_else(|_| Err(ParseColorError::GreenChannelOutOfBounds))?;
            let b = u8::from_str_radix(&hex_only[4..6], 16)
                .or_else(|_| Err(ParseColorError::BlueChannelOutOfBounds))?;
            Ok(Rgb { red: r, green: g, blue: b })
        } else {
            return Err(ParseColorError::NoLeadingHash);
        }

        // my original solution
        // didn't make use of channel out of bounds errors, but still passed the tests.
        // if !s.starts_with('#') {
        //     return Err(ParseColorError::NoLeadingHash);
        // }
        // let hex_only = s.trim_start_matches('#');
        // let buffer = <[u8; 3]>::from_hex(hex_only).unwrap();
        // Ok(Rgb { red: buffer[0], green: buffer[1], blue: buffer[2] })

    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    // 
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short () {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}

