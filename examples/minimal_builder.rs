use pixen::{EngineBuilder, PixelWindow};

fn draw(window: &mut PixelWindow) {
    for x in 0..window.width() {
        for y in 0..window.height() {
            if x % 2 == 0 || y % 2 != 0 {
                continue;
            }

            window.set_pixel(x, y, pixen::WHITE);
        }
    }
}

fn main() {
    // TODO: I don't love that we have to give an irrelevant type parameter here.
    EngineBuilder::<_, ()>::new(draw)
        .with_title("minimal builder")
        .with_width(100)
        .with_height(100)
        .build()
        .run();
}
