#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
mod engine;
mod pixel;

/// A simple type alias for an rgba color from 4 bytes.
/// Pixen itself is agnostic to the exact semantics of the color, so as long
/// as it can be represented as an array of 4 bytes, any color can be used.
pub type ColorRGBA = [u8; 4];

pub use engine::run_stateless;
pub use pixel::PixelWindow;
