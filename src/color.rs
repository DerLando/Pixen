/// A simple type alias for an rgba color from 4 bytes.
/// Pixen itself is agnostic to the exact semantics of the color, so as long
/// as it can be represented as an array of 4 bytes, any color can be used.:
pub type ColorRGBA = [u8; 4];

pub const BLACK: ColorRGBA = [0, 0, 0, 255];
pub const WHITE: ColorRGBA = [255, 255, 255, 255];
