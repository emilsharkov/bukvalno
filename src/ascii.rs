use image::ImageReader;
use std::error::Error;
use crate::{charsets, cli, constants::{CHAR_HEIGHT_TO_WIDTH_RATIO, MAX_RGB_VALUE}, utils::resize_img};

pub fn generate_ascii_art(config: &cli::Config) -> Result<String, Box<dyn Error>> {
    let cli::Config {filename, scale, charset_label, invert} = config;

    let charset = charsets::get_charset_from_label(&charset_label).unwrap().to_string();
    let ordered_charset: Vec<char> = if *invert { 
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
            let ascii_index = ((gray_average / MAX_RGB_VALUE) * (ordered_charset.len() - 1) as f32) as u8;
            let ascii_char = ordered_charset.get(ascii_index as usize).unwrap();
            line.push(*ascii_char);
        }
        line.push('\n');
        lines.push_str(&line);
    }
    Ok(lines)
}
