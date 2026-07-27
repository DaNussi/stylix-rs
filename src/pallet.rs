use serde::{Deserialize, Serialize};

use crate::{color::Color, error::StylixError};

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct RawStylixPallet {
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

#[derive(Default)]
pub struct StylixPallet {
    base00: Color,
    base01: Color,
    base02: Color,
    base03: Color,
    base04: Color,
    base05: Color,
    base06: Color,
    base07: Color,
    base08: Color,
    base09: Color,
    base0A: Color,
    base0B: Color,
    base0C: Color,
    base0D: Color,
    base0E: Color,
    base0F: Color,
    author: String,
    scheme: String,
    slug: String,
}

impl StylixPallet {
    pub(crate) fn parse(value: RawStylixPallet) -> Result<StylixPallet, StylixError> {
        Ok(StylixPallet {
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
