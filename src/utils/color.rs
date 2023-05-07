use std::{fs::File, io::Write};

pub const MAX_COLOR_VAL: f64 = 255.0;

#[allow(dead_code)]
pub fn write_color(pixel_color: super::Color) {
    let vr = (MAX_COLOR_VAL * pixel_color.x.clamp(0.0, 0.9999)) as i64;
    let vg = (MAX_COLOR_VAL * pixel_color.y.clamp(0.0, 0.9999)) as i64;
    let vb = (MAX_COLOR_VAL * pixel_color.z.clamp(0.0, 0.9999)) as i64;
    print!("{vr}\t{vg}\t{vb}\t");
}

pub fn write_color_to_buffer(pixel_color: super::Color, mut buffer: &File) -> std::io::Result<()> {
    let vr = (MAX_COLOR_VAL * pixel_color.x.clamp(0.0, 0.9999)) as i64;
    let vg = (MAX_COLOR_VAL * pixel_color.y.clamp(0.0, 0.9999)) as i64;
    let vb = (MAX_COLOR_VAL * pixel_color.z.clamp(0.0, 0.9999)) as i64;

    buffer.write_fmt(format_args!("{}\t{}\t{}\n", vr, vg, vb))?;

    Ok(())
}
