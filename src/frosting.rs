//! Ganache frosting, poured over a sponge.
use crate::sponge::Sponge;

/// A frosted sponge.
#[derive(Debug, Clone, PartialEq)]
pub struct Frosted {
    pub sponge: Sponge,
    /// Millimetres of ganache on top.
    pub ganache_mm: u32,
}

/// Pour ganache over a sponge. More cocoa takes thicker ganache.
pub fn frost(sponge: Sponge) -> Frosted {
    let ganache_mm = 4 + sponge.cocoa_g / 100;
    Frosted { sponge, ganache_mm }
}
