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

fn pixel_to_brightness(pixel: Rgba<u8>) -> u8 {
    // Rec standard for luma grayscale conversion
    (pixel[0] as f32 * 0.3 + pixel[1] as f32 * 0.59 + pixel[2] as f32 * 0.11) as u8
}

fn pixel_to_ascii(pixel: Rgba<u8>) -> char {
    let brightness = pixel_to_brightness(pixel);
    let char_idx = (brightness as f32 / 255.0 * (ASCII_CHARS.len() - 1) as f32) as usize;
    let ascii_char = ASCII_CHARS.chars().nth(char_idx).unwrap();

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

// From darkest to lightest
const ASCII_CHARS: &str = "@%#*+=-:. ";

fn main() {
    let args = Cli::parse();
    let img = image::open(&args.path).expect(&format!("Failed to open image {:?}", args.path));

    let width = args.width;
    let height = args.height;

    let img = img.resize(width, height, image::imageops::FilterType::Nearest);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            let ascii_char = pixel_to_ascii(pixel);
            let color = pixel_to_color(pixel, &args.color_scheme);

            let color_ascii = colorize_ascii(ascii_char, color);

            print!("{}", color_ascii);
        }
        println!();
    }
}
