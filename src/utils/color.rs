use super::vector;

pub const MAX_COLOR_VAL: f64 = 255.0;

pub fn write_color(pixel_color: vector::Color) {
    let vr = (MAX_COLOR_VAL * pixel_color.x.clamp(0.0, 0.9999)) as i64;
    let vg = (MAX_COLOR_VAL * pixel_color.y.clamp(0.0, 0.9999)) as i64;
    let vb = (MAX_COLOR_VAL * pixel_color.z.clamp(0.0, 0.9999)) as i64;
    println!("{vr}\t{vg}\t{vb}");
}
