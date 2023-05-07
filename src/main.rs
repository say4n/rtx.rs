mod physics;
mod utils;
mod world;

use utils::MAX_COLOR_VAL;

const IMAGE_FORMAT: &str = "P3"; // PPM, ASCII, RGB
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
    let camera = world::Camera::default();

    println!("{IMAGE_FORMAT}\n{IMAGE_WIDTH}\t{IMAGE_HEIGHT}\n{MAX_COLOR_VAL}");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\rRender progress {progress}%.",
            progress = 100 * (1 - j / IMAGE_HEIGHT)
        );

        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = physics::Ray::new(
                camera.origin,
                camera.lower_left_corner + u * camera.horizontal + v * camera.vertical
                    - camera.origin,
            );
            let pixel = r.color();
            utils::write_color(pixel);
        }
        print!("\n")
    }
    eprintln!("\nDone.");
}
