use crate::config::ColorScheme;
use colored::Colorize;
use image::{Rgb, Rgba};

const PASTEL_FACTOR: f32 = 0.7;
fn pastellize_pixel(pixel: Rgba<u8>) -> Rgb<u8> {
    // Mix with white to create pastel version
    // Formula: new_color = original_color * 0.7 + 255 * 0.3
    Rgb([
        ((pixel[0] as f32 * PASTEL_FACTOR) + (255.0 * (1.0 - PASTEL_FACTOR))) as u8,
        ((pixel[1] as f32 * PASTEL_FACTOR) + (255.0 * (1.0 - PASTEL_FACTOR))) as u8,
        ((pixel[2] as f32 * PASTEL_FACTOR) + (255.0 * (1.0 - PASTEL_FACTOR))) as u8,
    ])
}

pub fn pixel_to_color(pixel: Rgba<u8>, scheme: &ColorScheme) -> Rgb<u8> {
    match scheme {
        ColorScheme::Original => Rgb([pixel[0], pixel[1], pixel[2]]),
        ColorScheme::BlackAndWhite => Rgb([0, 0, 0]),
        ColorScheme::Pastel => pastellize_pixel(pixel),
    }
}

pub fn colorize_ascii(c: char, color: Rgb<u8>) -> String {
    c.to_string()
        .truecolor(color[0], color[1], color[2])
        .to_string()
}
