use crate::ColorRGBA;
use std::error::Error;

/// A safe wrapper around a continuous slice of raw pixel bytes.
/// The inner color values of the pixels can be accessed by giving
/// x and y coordinates instead of raw index access.
pub struct PixelWindow<'a> {
    width: u32,
    height: u32,
    raw_buffer: &'a mut [u8],
}

impl<'a> PixelWindow<'a> {
    /// Create a new [`PixelWindow`] which wraps the given buffer. Buffers are stored
    /// as mutable references, so ownership of the buffer can reside with the caller.
    pub(crate) fn new(width: u32, height: u32, raw_buffer: &'a mut [u8]) -> Self {
        assert!(raw_buffer.len() == (width * height * 4) as usize);
        Self {
            width,
            height,
            raw_buffer,
        }
    }

    #[inline]
    fn xy_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width * 4 + x * 4) as usize
    }

    #[inline]
    fn write_to_buffer(&mut self, start: usize, bytes: &[u8]) {
        self.raw_buffer[start..start + bytes.len()].copy_from_slice(bytes);
    }

    /// The width of the [`PixelWindow`] in pixels
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the [`PixelWindow`] in pixels
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the pixel at the given x and y location to the given [`ColorRGBA`].
    /// This function is save in regards to window bounds and will clamp the x and y inputs.
    /// If you know the location is in bounds, consider using [`PixelWindow::set_pixel_unchecked()`] instead.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: ColorRGBA) {
        let x = x % self.width;
        let y = y % self.height;

        self.set_pixel_unchecked(x, y, color)
    }

    /// Set the pixel at the given x and y location to the given [`ColorRGBA`].
    /// This function **will panic** if either the x or y inputs are outside the [`PixelWindow`]s bounds.
    pub fn set_pixel_unchecked(&mut self, x: u32, y: u32, color: ColorRGBA) {
        let index = self.xy_to_index(x, y);
        self.write_to_buffer(index, &color)
    }

    /// Get the pixel color at the given x and y location.
    /// This function is save in regards to window bounds and will clamp the x and y inputs.
    /// If you know the location is in bounds, consider using [`PixelWindow::get_pixel_unchecked()`] instead.
    pub fn get_pixel(&self, x: u32, y: u32) -> &ColorRGBA {
        let x = x % self.width;
        let y = y % self.height;

        self.get_pixel_unchecked(x, y)
    }

    /// Get the pixel color at the given x and y location.
    /// This function **will panic** if either the x or y inputs are outside the [`PixelWindow`]s bounds.
    pub fn get_pixel_unchecked(&self, x: u32, y: u32) -> &ColorRGBA {
        let index = self.xy_to_index(x, y);

        match self.raw_buffer[index..index + 4].try_into() {
            Ok(p) => p,
            Err(e) => {
                panic!();
            }
        }
    }

    /// Clear the whole window to the given color
    pub fn clear(&mut self, color: ColorRGBA) {
        for chunk in self.raw_buffer.chunks_exact_mut(4) {
            chunk.copy_from_slice(&color);
        }
    }

    /// Consume the [`PixelWindow`] and give back mut access to it's
    /// inner, raw pixel bytes buffer
    pub fn into_inner(self) -> &'a mut [u8] {
        self.raw_buffer
    }
}

#[cfg(feature = "image")]
impl<'a> PixelWindow<'a> {
    /// Capture the current state of the [`PixelWindow`] to a an [`image::ImageBuffer`].
    /// This buffer can be saved to a file safely.
    ///
    /// # Performance
    ///
    /// This effectively copies the inner buffer fully, so it is quite costly.
    pub fn capture_to_image(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        image::ImageBuffer::from_fn(self.width - 1, self.height - 1, |x, y| {
            image::Rgba(*self.get_pixel_unchecked(x, y))
        })
    }
}
