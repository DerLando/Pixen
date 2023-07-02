use pixen::run_statefull;
use pixen::PixelWindow;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const RADIUS: f32 = 25.0;

fn draw(window: &mut PixelWindow, angle: &f32) {
    let x = RADIUS * angle.cos() + (WIDTH / 2) as f32;
    let y = RADIUS * angle.sin() + (HEIGHT / 2) as f32;

    // Clear the whole window to black
    window.clear([0, 0, 0, 0]);
    // Draw a point in polar coordinates
    window.set_pixel(x as u32, y as u32, pixen::WHITE);
}

fn main() {
    run_statefull(WIDTH, HEIGHT, 0.0, draw, |angle| {
        *angle = *angle + 0.05;
    })
    .expect("Valid draw fn");
}
