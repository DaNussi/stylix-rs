//! This is my awesome crate
//!
//! SOmething SOmething 1
//!
//! # Examples
//! ```
//! fn sum2(n1: i32, n2: i32) -> i32 {
//!   n1 + n2
//! }
//! # assert_eq!(4, sum2(2, 2));
//! ```
//!
use home_config::HomeConfig;

use crate::{
    error::StylixError,
    pallet::{RawStylixPallet, StylixPallet},
};

mod color;
mod error;
mod pallet;

pub struct Stylix;

impl Stylix {
    pub fn load() -> Result<StylixPallet, StylixError> {
        let raw_pallet: RawStylixPallet = HomeConfig::with_config_dir("stylix", "palette.json")
            .json()
            .map_err(|e| StylixError::ConfigLoadError(e))?;

        StylixPallet::parse(raw_pallet)
    }
}
