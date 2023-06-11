#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
mod engine;
mod pixel;

pub type Color = [u8; 4];

pub use engine::run_stateless;
