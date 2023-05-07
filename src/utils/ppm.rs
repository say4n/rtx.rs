use std::{fs::File, io::Write};

use super::{write_color_to_buffer, Color, MAX_COLOR_VAL};

pub struct PPMImage {
    pub height: i32,
    pub width: i32,
}

impl PPMImage {
    pub fn write(&self, pixel_buffer: Vec<Color>, filename: &str) -> std::io::Result<()> {
        let mut buffer = File::create(filename)?;

        buffer.write(b"P3\n")?;
        buffer.write_fmt(format_args!("{}\t{}\n", self.width, self.height))?;
        buffer.write_fmt(format_args!("{}\n", MAX_COLOR_VAL))?;

        for pixel in pixel_buffer.iter().rev() {
            write_color_to_buffer(*pixel, &buffer)?;
        }

        Ok(())
    }
}
