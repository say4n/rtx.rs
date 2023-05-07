mod utils;
use utils::MAX_COLOR_VAL;

fn main() {
    let image_format = "P3"; // PPM, ASCII, RGB
    let image_height = 256;
    let image_width = 256;

    println!("{image_format}\n{image_height}\t{image_width}\n{MAX_COLOR_VAL}");

    for i in (0..image_height).rev() {
        eprint!(
            "\rRender progress {progress}%.",
            progress = 100 * (1 - i / image_height)
        );

        for j in 0..image_width {
            let pixel = utils::Color {
                x: i as f64 / image_height as f64,
                y: j as f64 / image_width as f64,
                z: 0.25,
            };
            utils::write_color(pixel);
        }
        println!("")
    }
    eprintln!("\nDone.");
}
