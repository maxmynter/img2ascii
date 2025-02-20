use clap::{Parser, ValueEnum};
use colored::Colorize;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::software::scaling::{self, Context};
use image::{GenericImageView, Rgb, Rgba};
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};
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

struct VideoContext {
    input: ffmpeg::format::context::Input,
    decoder: ffmpeg::decoder::Video,
    frame_duration: Duration,
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
    // Edge detection with curves ?

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
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

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
    target_width: u32,
    target_height: u32,
) -> Result<image::DynamicImage, Box<dyn Error>> {
    let mut rgb_frame =
        ffmpeg::frame::Video::new(ffmpeg::format::Pixel::RGB24, target_width, target_height);

    let sws_context = Context::get(
        frame.format(),
        frame.width(),
        frame.height(),
        ffmpeg::format::Pixel::RGB24,
        target_width,
        target_height,
        scaling::Flags::BILINEAR,
    );

    sws_context?.run(frame, &mut rgb_frame)?;

    let buffer =
        image::ImageBuffer::from_raw(target_width, target_height, rgb_frame.data(0).to_vec())
            .ok_or("Failed to create image buffer")?;

    Ok(image::DynamicImage::ImageRgb8(buffer))
}

fn init_video(path: &std::path::Path) -> Result<VideoContext, Box<dyn Error>> {
    ffmpeg::init()?;

    let input = ffmpeg::format::input(&path)?;
    let stream = input
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or("No video stream found")?;

    let frame_rate = stream.rate();
    let frame_duration =
        Duration::from_secs_f64(frame_rate.denominator() as f64 / frame_rate.numerator() as f64);

    let context = stream.parameters();
    let decoder = ffmpeg::codec::Context::from_parameters(context)?
        .decoder()
        .video()?;

    Ok(VideoContext {
        input,
        decoder,
        frame_duration,
    })
}

fn process_video(args: Cli) -> Result<(), Box<dyn Error>> {
    let VideoContext {
        mut input,
        mut decoder,
        frame_duration,
    } = init_video(&args.path)?;

    let mut frame = ffmpeg::frame::Video::empty();
    let mut stdout = io::stdout().lock();
    let mut frame_timer = Instant::now();

    let video_stream_index = input
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or("No video stream found")?
        .index();

    for (stream, packet) in input.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            while decoder.receive_frame(&mut frame).is_ok() {
                let image = frame_to_dynamic_image(&frame, args.width, args.height)?;
                let ascii = generate_ascii_art(
                    &image,
                    args.width,
                    args.height,
                    &args.color_scheme,
                    args.granularity,
                )?;
                write!(stdout, "\x1B[2J\x1B[1;1H")?;
                write!(stdout, "{}", ascii)?;
                stdout.flush()?;
                let elapsed = frame_timer.elapsed();
                if elapsed < frame_duration {
                    std::thread::sleep(frame_duration - elapsed);
                }
                frame_timer = Instant::now();
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match detect_input_media_type(&args.path)? {
        MediaInput::Image(_path) => {
            let ascii_art = generate_ascii_image(args)?;
            print!("{}", ascii_art);
        }
        MediaInput::Video(_path) => {
            process_video(args)?;
        }
    }

    Ok(())
}
