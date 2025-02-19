use clap::{Parser, ValueEnum};
use colored::Colorize;
use image::{GenericImageView, Rgb, Rgba};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[arg(short = 'x', long, default_value_t = 100)]
    width: u32,

    #[arg(short = 'y', long, default_value_t = 100)]
    height: u32,

    #[arg(short = 'c', long, default_value_t = ColorScheme::Original)]
    color_scheme: ColorScheme,

    #[arg(short= 'g', long, default_value_t = 1.0,value_parser=validate_granularity )]
    granularity: f32,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ColorScheme {
    Original,
    BlackAndWhite,
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
    }
}

fn colorize_ascii(c: char, color: Rgb<u8>) -> String {
    c.to_string()
        .truecolor(color[0], color[1], color[2])
        .to_string()
}

fn main() {
    let args = Cli::parse();
    let img = image::open(&args.path).expect(&format!("Failed to open image {:?}", args.path));

    let width = args.width;
    let height = args.height;

    let img = img.resize(width, height, image::imageops::FilterType::Nearest);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            let ascii_set = get_ascii_set(args.granularity);
            let ascii_char = pixel_to_ascii(pixel, &ascii_set);

            let color = pixel_to_color(pixel, &args.color_scheme);

            let color_ascii = colorize_ascii(ascii_char, color);

            print!("{}", color_ascii);
        }
        println!();
    }
}
