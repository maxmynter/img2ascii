use clap::Parser;
use image::{GenericImageView, ImageBuffer};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    // Width and height can be cli args too
}

// From darkest to lightest
const ASCII_CHARS: &str = "@%#*+=-:. ";

fn main() {
    let args = Cli::parse();
    let img = image::open(&args.path).expect(&format!("Failed to open image {:?}", args.path));

    let width = 100;
    let height = 100;

    let img = img.resize(width, height, image::imageops::FilterType::Nearest);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            // Rec standard for luma grayscale conversion
            let brightness =
                (pixel[0] as f32 * 0.3 + pixel[1] as f32 * 0.59 + pixel[2] as f32 * 0.11) as u8;
            let char_idx = (brightness as f32 / 255.0 * (ASCII_CHARS.len() - 1) as f32) as usize;

            let ascii_char = ASCII_CHARS.chars().nth(char_idx).unwrap();
            print!("{}", ascii_char);
        }
        println!();
    }
}
