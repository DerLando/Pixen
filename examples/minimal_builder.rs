use pixen::{EngineBuilder, PixelWindow};

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

fn draw(window: &mut PixelWindow) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if x % 2 == 0 || y % 2 != 0 {
                continue;
            }

            window.set_pixel(x, y, pixen::WHITE);
        }
    }
}

fn main() {
    EngineBuilder::<_, ()>::new(draw)
        .with_width(WIDTH)
        .with_height(HEIGHT)
        .build()
        .run();
}
