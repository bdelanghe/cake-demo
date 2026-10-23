//! The sponge: every cake starts here.
use crate::Bake;

/// A chocolate sponge layer.
#[derive(Debug, Clone, PartialEq)]
pub struct Sponge {
    /// Grams of cocoa.
    pub cocoa_g: u32,
}

impl Bake for Sponge {
    fn minutes(&self) -> u32 {
        30 + self.cocoa_g / 50
    }
}
