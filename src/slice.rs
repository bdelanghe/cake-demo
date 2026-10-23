//! Cutting a frosted cake into slices: everyone slices differently.
use crate::frosting::Frosted;

/// One slice, as a share of the whole cake.
#[derive(Debug, Clone, PartialEq)]
pub struct Slice {
    pub share: f64,
}

/// Cut into `n` equal slices. Every crumb lands in exactly one slice.
pub fn cut(_cake: &Frosted, n: usize) -> Vec<Slice> {
    assert!(n > 0, "a cake needs at least one slice");
    vec![Slice { share: 1.0 / n as f64 }; n]
}
