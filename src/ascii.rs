use image::ImageReader;
use std::error::Error;
use crate::{cli::Config, constants::CHAR_HEIGHT_TO_WIDTH_RATIO, utils::resize_img};

pub fn generate_ascii_art(config: &Config) -> Result<String, Box<dyn Error>> {
    let Config {filename, scale, charset, invert} = config;

    let charset: String = if *invert { 
        charset.chars().rev().collect() 
    } else { 
        charset.chars().collect() 
    };

    let mut img = ImageReader::open(filename)?.decode()?;
    img = resize_img(&img, 1.0, CHAR_HEIGHT_TO_WIDTH_RATIO);
    img = resize_img(&img, *scale, *scale);

    let mut lines = String::new();
    let rgb_buffer = img.as_rgb8().unwrap();
    for row in rgb_buffer.rows() {
        let mut line = String::new();
        for pixel in row {
            let gray_average = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
            let ascii_index = (gray_average / 4.0) as u8;
            let ascii_char = charset.chars().nth(ascii_index as usize).unwrap();
            line.push(ascii_char);
        }
        line.push('\n');
        lines.push_str(&line);
    }
    Ok(lines)
}
