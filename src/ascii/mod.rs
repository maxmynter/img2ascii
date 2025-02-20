mod color;
use crate::config::ColorScheme;
pub use color::*;
use image::{GenericImageView, Rgba};
use std::error::Error;

fn get_ascii_set(granularity: f32) -> String {
    // From darkest to lightest
    const FULL_ASCII: &str =
    "@&%QWNM0gB$#DR8mHXKAUbGOpV4d9h6PkqwSE2]ayjxY5Zoenult13If}C{iF|()7Jv)TLs?z*/cr!+<>;=^,_:'.-` ";

    let num_chars = (1.0 + (FULL_ASCII.len() - 1) as f32 * granularity).round() as usize;

    if num_chars >= FULL_ASCII.len() {
        return FULL_ASCII.to_string();
    } else {
        return FULL_ASCII
            .chars()
            .step_by(FULL_ASCII.len() / num_chars)
            .take(num_chars)
            .collect();
    }
}
fn pixel_to_brightness(pixel: Rgba<u8>) -> u8 {
    // Rec standard for luma grayscale conversion
    (pixel[0] as f32 * 0.3 + pixel[1] as f32 * 0.59 + pixel[2] as f32 * 0.11) as u8
}

fn pixel_to_ascii(pixel: Rgba<u8>, ascii_set: &String) -> char {
    let brightness = pixel_to_brightness(pixel);
    let char_idx = (brightness as f32 / 255.0 * (ascii_set.len() - 1) as f32) as usize;
    let ascii_char = ascii_set.chars().nth(char_idx).unwrap();

    ascii_char
}
pub fn generate_ascii_art(
    img: &image::DynamicImage,
    width: u32,
    height: u32,
    scheme: &ColorScheme,
    granularity: f32,
) -> Result<String, Box<dyn Error>> {
    let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    let ascii_set = get_ascii_set(granularity);

    let mut output = String::with_capacity((width * height) as usize);

    for y in 0..resized.height() {
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);

            let ascii_char = pixel_to_ascii(pixel, &ascii_set);

            let color = pixel_to_color(pixel, &scheme);
            let color_ascii = colorize_ascii(ascii_char, color);

            output.push_str(&color_ascii);
        }
        output.push('\n')
    }
    Ok(output)
}
