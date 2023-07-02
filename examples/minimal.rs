use pixen::run_stateless;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

fn main() {
    run_stateless(WIDTH, HEIGHT, |window| {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if x % 2 == 0 || y % 2 != 0 {
                    continue;
                }

                window.set_pixel(x, y, pixen::WHITE);
            }
        }
    })
    .expect("Should not error");
}
