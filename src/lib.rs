#![deny(clippy::all)]
#![forbid(unsafe_code)]
mod engine;
mod pixel;

pub type Color = [u8; 4];

pub use engine::run_stateless;
