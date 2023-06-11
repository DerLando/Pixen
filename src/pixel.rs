use crate::Color;

pub struct PixelWindow<'a> {
    width: u32,
    height: u32,
    raw_buffer: &'a mut [u8],
}

impl<'a> PixelWindow<'a> {
    pub fn new(width: u32, height: u32, raw_buffer: &'a mut [u8]) -> Self {
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

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let x = x % self.width;
        let y = y % self.height;

        self.set_pixel_unchecked(x, y, color)
    }

    pub fn set_pixel_unchecked(&mut self, x: u32, y: u32, color: Color) {
        let index = self.xy_to_index(x, y);
        self.write_to_buffer(index, &color)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let x = x % self.width;
        let y = y % self.height;

        self.get_pixel_unchecked(x, y)
    }

    pub fn get_pixel_unchecked(&self, x: u32, y: u32) -> Color {
        let index = self.xy_to_index(x, y);
        self.raw_buffer[index..=index + 4]
            .try_into()
            .expect("Buffer is big enough")
    }
}
