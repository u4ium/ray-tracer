use crate::image::Image;
use std::fs::File;
use std::io::{self, Write};

pub fn write_to_ppm(image: Image, filename: &str) -> io::Result<()> {
    let mut image_file = File::create(filename)?;
    let width = image.pixels.ncols();
    let height = image.pixels.nrows();
    write!(image_file, "P3\n{} {}\n255\n", width, height)?;
    for row in image.pixels.rows() {
        for pixel in row {
            let colour = pixel.to_colour().to_string();
            write!(image_file, "{}\t", colour)?;
        }
        write!(image_file, "\n")?;
    }
    Ok(())
}
