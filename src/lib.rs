//! bakery: a toy crate for the cake demo.

pub mod sponge;

/// Something you can bake.
pub trait Bake {
    /// Minutes in the oven.
    fn minutes(&self) -> u32;
}
