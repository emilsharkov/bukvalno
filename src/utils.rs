use image::{DynamicImage, GenericImageView};

pub fn resize_img(img: &DynamicImage, width_scale: f32, height_scale: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let new_width = (width as f32 / width_scale) as u32;
    let new_height = (height as f32 / height_scale) as u32;
    img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest)
}
