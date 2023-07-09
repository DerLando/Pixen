#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "window")]
mod engine;

#[cfg(feature = "window")]
pub use engine::{run_statefull, run_stateless, EngineBuilder, PixenEngine};

mod pixel;

mod color;

pub use color::{ColorRGBA, BLACK, WHITE};
pub use pixel::PixelWindow;
