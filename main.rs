mod vector;

fn main() {
    let image_format = "P3"; // PPM, ASCII, RGB
    let image_height = 256;
    let image_width = 256;
    let max_color_val = 255;

    println!("{image_format}\n{image_height}\t{image_width}\n{max_color_val}");

    for i in (0..image_height).rev() {
        eprint!(
            "\rRender progress {progress}%.",
            progress = 100 * (1 - i / image_height)
        );

        for j in 0..image_width {
            let vr = (i * max_color_val) / image_height;
            let vg = (j * max_color_val) / image_width;
            let vb = 128;

            println!("{vr}\t{vg}\t{vb}")
        }
        println!("")
    }
    eprintln!("\nDone.");
}
