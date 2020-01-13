use std::io;
use std::path::Path;
use std::fs::File;

extern crate png;

use png::png::HasParameters;

pub fn save_png(filename: &str, width: u32, height: u32, buffer: &[u8]) -> io::Result<()> {
    let tmp = filename;
    let path = Path::new(&tmp);
    let file = File::create(path)?;
    let w = io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(buffer)?; // Save

    Ok(())
}
