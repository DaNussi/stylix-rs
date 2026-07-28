use std::{fmt::Display, ops::Range};

use crate::error::{ParseColorError, StylixError};

#[derive(Default, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(&format!("Color: r{} g{} b{}", self.r, self.g, self.b));
        Ok(())
    }
}

impl TryFrom<&str> for Color {
    type Error = StylixError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let has_prefix = value.starts_with('#');
        let expected_length = if has_prefix { 7 } else { 6 };

        if value.len() != expected_length {
            return Err(ParseColorError::InvalidLength {
                value: value.to_owned(),
                length: value.len(),
            }
            .into_stylix_error());
        }

        let rgb_indexes = if has_prefix {
            [1..3, 3..5, 5..7]
        } else {
            [0..2, 2..4, 4..6]
        };

        let r = Color::u8_from_str(value, rgb_indexes[0].clone())?;
        let g = Color::u8_from_str(value, rgb_indexes[1].clone())?;
        let b = Color::u8_from_str(value, rgb_indexes[2].clone())?;

        Ok(Color { r, g, b })
    }
}

impl Color {
    fn u8_from_str(value: &str, range: Range<usize>) -> Result<u8, StylixError> {
        let index_out_of_range_error = ParseColorError::IndexOutOfRange {
            value: value.to_owned(),
            range: range.clone(),
        }
        .into_stylix_error();

        let parse_int_error = |e| {
            ParseColorError::ParseIntError {
                value: value.to_owned(),
                error: e,
            }
            .into_stylix_error()
        };

        let text_part = value.get(range.clone()).ok_or(index_out_of_range_error)?;
        u8::from_str_radix(text_part, 16).map_err(parse_int_error)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

#[cfg(feature = "ratatui-color")]
pub mod ratatui_color {
    use crate::color::Color;
    use ratatui_core::style::Color as RatatuiColor;

    impl From<Color> for RatatuiColor {
        fn from(value: Color) -> Self {
            RatatuiColor::Rgb(value.r, value.g, value.b)
        }
    }

    #[test]
    fn ratatui_conversion() {
        let color = Color {
            r: 255,
            g: 255,
            b: 255,
        };
        let ratatui_color: RatatuiColor = color.into();
        assert_eq!(ratatui_color, RatatuiColor::Rgb(255, 255, 255));
    }
}
