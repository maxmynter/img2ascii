use clap::{Parser, ValueEnum};
use image::GenericImageView;

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
