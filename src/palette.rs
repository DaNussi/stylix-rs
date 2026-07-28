use serde::{Deserialize, Serialize};

use crate::{color::Color, error::StylixError};

#[derive(Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub(crate) struct RawStylixPalette {
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    base0A: String,
    base0B: String,
    base0C: String,
    base0D: String,
    base0E: String,
    base0F: String,
    author: String,
    scheme: String,
    slug: String,
}

#[derive(Default, Debug)]
#[allow(non_snake_case)]
pub struct StylixPalette {
    pub base00: Color,
    pub base01: Color,
    pub base02: Color,
    pub base03: Color,
    pub base04: Color,
    pub base05: Color,
    pub base06: Color,
    pub base07: Color,
    pub base08: Color,
    pub base09: Color,
    pub base0A: Color,
    pub base0B: Color,
    pub base0C: Color,
    pub base0D: Color,
    pub base0E: Color,
    pub base0F: Color,
    pub author: String,
    pub scheme: String,
    pub slug: String,
}

impl StylixPalette {
    pub(crate) fn parse(value: RawStylixPalette) -> Result<StylixPalette, StylixError> {
        Ok(StylixPalette {
            base00: Color::try_from(value.base00.as_str())?,
            base01: Color::try_from(value.base01.as_str())?,
            base02: Color::try_from(value.base02.as_str())?,
            base03: Color::try_from(value.base03.as_str())?,
            base04: Color::try_from(value.base04.as_str())?,
            base05: Color::try_from(value.base05.as_str())?,
            base06: Color::try_from(value.base06.as_str())?,
            base07: Color::try_from(value.base07.as_str())?,
            base08: Color::try_from(value.base08.as_str())?,
            base09: Color::try_from(value.base09.as_str())?,
            base0A: Color::try_from(value.base0A.as_str())?,
            base0B: Color::try_from(value.base0B.as_str())?,
            base0C: Color::try_from(value.base0C.as_str())?,
            base0D: Color::try_from(value.base0D.as_str())?,
            base0E: Color::try_from(value.base0E.as_str())?,
            base0F: Color::try_from(value.base0F.as_str())?,
            author: value.author,
            scheme: value.scheme,
            slug: value.slug,
        })
    }
}

impl PartialEq for StylixPalette {
    fn eq(&self, other: &Self) -> bool {
        self.base00 == other.base00
            && self.base01 == other.base01
            && self.base02 == other.base02
            && self.base03 == other.base03
            && self.base04 == other.base04
            && self.base05 == other.base05
            && self.base06 == other.base06
            && self.base07 == other.base07
            && self.base08 == other.base08
            && self.base09 == other.base09
            && self.base0A == other.base0A
            && self.base0B == other.base0B
            && self.base0C == other.base0C
            && self.base0D == other.base0D
            && self.base0E == other.base0E
            && self.base0F == other.base0F
            && self.author == other.author
            && self.scheme == other.scheme
            && self.slug == other.slug
    }
}
