use clap::{Parser, ValueEnum};
use colored::Colorize;
use ffmpeg_next as ffmpeg;
use image::{GenericImageView, Rgb, Rgba};
use std::path::Path;
use std::{error::Error, path::PathBuf};

const DEFAULT_DIMENSION: u32 = 100;
const PASTEL_FACTOR: f32 = 0.7;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[arg(short = 'x', long, default_value_t = DEFAULT_DIMENSION)]
    width: u32,

    #[arg(short = 'y', long, default_value_t = DEFAULT_DIMENSION)]
    height: u32,

    #[arg(short = 'c', long, default_value_t = ColorScheme::Original)]
    color_scheme: ColorScheme,

    #[arg(short= 'g', long, default_value_t = 1.0,value_parser=validate_granularity )]
    granularity: f32,
}

enum MediaInput {
    Image(PathBuf),
    Video(PathBuf),
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ColorScheme {
    Original,
    BlackAndWhite,
    Pastel,
}

impl std::fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values skipped")
            .get_name()
            .fmt(f)
    }
}

fn validate_granularity(s: &str) -> Result<f32, String> {
    let val = s
        .parse::<f32>()
        .map_err(|_| "Must be a number".to_string())?;
    if !(0.0..=1.0).contains(&val) {
        return Err("Granularity must be in 0..1".to_string());
    } else {
        Ok(val)
    }
}

fn get_ascii_set(granularity: f32) -> String {
    // From darkest to lightest
    const FULL_ASCII: &str =
        "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

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

fn pastellize_pixel(pixel: Rgba<u8>) -> Rgb<u8> {
    // Mix with white to create pastel version
    // Formula: new_color = original_color * 0.7 + 255 * 0.3
    Rgb([
        ((pixel[0] as f32 * PASTEL_FACTOR) + (255.0 * (1.0 - PASTEL_FACTOR))) as u8,
        ((pixel[1] as f32 * PASTEL_FACTOR) + (255.0 * (1.0 - PASTEL_FACTOR))) as u8,
        ((pixel[2] as f32 * PASTEL_FACTOR) + (255.0 * (1.0 - PASTEL_FACTOR))) as u8,
    ])
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

fn pixel_to_color(pixel: Rgba<u8>, scheme: &ColorScheme) -> Rgb<u8> {
    match scheme {
        ColorScheme::Original => Rgb([pixel[0], pixel[1], pixel[2]]),
        ColorScheme::BlackAndWhite => Rgb([0, 0, 0]),
        ColorScheme::Pastel => pastellize_pixel(pixel),
    }
}

fn colorize_ascii(c: char, color: Rgb<u8>) -> String {
    c.to_string()
        .truecolor(color[0], color[1], color[2])
        .to_string()
}

fn detect_input_media_type(path: &Path) -> Result<MediaInput, Box<dyn Error>> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("mp4" | "avi" | "mov" | "mkv") => Ok(MediaInput::Video(path.to_path_buf())),
        Some("jpg" | "jpeg" | "png" | "gif") => Ok(MediaInput::Image(path.to_path_buf())),
        _ => Err("Unsupported Media type".into()),
    }
}

fn generate_ascii_art(
    img: &image::DynamicImage,
    width: u32,
    height: u32,
    scheme: &ColorScheme,
    granularity: f32,
) -> Result<String, Box<dyn Error>> {
    let img = img.resize(width, height, image::imageops::FilterType::Nearest);

    let ascii_set = get_ascii_set(granularity);

    let mut output = String::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            let ascii_char = pixel_to_ascii(pixel, &ascii_set);

            let color = pixel_to_color(pixel, &scheme);
            let color_ascii = colorize_ascii(ascii_char, color);

            output.push_str(&color_ascii);
        }
        output.push('\n')
    }
    Ok(output)
}

fn generate_ascii_image(args: Cli) -> Result<String, Box<dyn Error>> {
    let img = image::open(&args.path)
        .map_err(|err| format!("Failed to open image {:?}, {}", args.path, err))?;

    generate_ascii_art(
        &img,
        args.width,
        args.height,
        &args.color_scheme,
        args.granularity,
    )
}

fn frame_to_dynamic_image(
    frame: &ffmpeg::frame::Video,
) -> Result<image::DynamicImage, Box<dyn Error>> {
    todo!("Not yet implemented")
}

fn init_video(
    path: &std::path::Path,
) -> Result<(ffmpeg::format::context::Input, ffmpeg::decoder::Video), Box<dyn Error>> {
    todo!("Not yet implemented")
}

fn process_video(args: Cli) -> Result<(), Box<dyn Error>> {
    todo!("Not yet implemented")
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match detect_input_media_type(&args.path)? {
        MediaInput::Image(_path) => {
            let ascii_art = generate_ascii_image(args)?;
            print!("{}", ascii_art);
        }
        MediaInput::Video(_path) => {
            todo!("Video Processing")
        }
    }

    Ok(())
}
