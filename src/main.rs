mod physics;
mod utils;
mod world;

use crate::utils::Color;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() -> std::io::Result<()> {
    let camera = world::Camera::default();
    let image = utils::PPMImage {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
    };

    let mut buffer = vec![Color::ZERO; (IMAGE_HEIGHT * IMAGE_WIDTH) as usize];

    for j in (0..IMAGE_HEIGHT).rev() {
        print!(
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

            buffer[(i + j * IMAGE_WIDTH) as usize] = r.color();
        }
    }

    image.write(buffer, "image.ppm")?;

    println!("\nDone.");

    Ok(())
}
