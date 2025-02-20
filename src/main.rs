mod ascii;
mod config;
mod media;

use clap::Parser;
use config::Cli;
use media::{generate_ascii_image, process_video};
use std::path::Path;
use std::{error::Error, path::PathBuf};

enum MediaInput {
    Image(PathBuf),
    Video(PathBuf),
}

fn detect_input_media_type(path: &Path) -> Result<MediaInput, Box<dyn Error>> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("mp4" | "avi" | "mov" | "mkv") => Ok(MediaInput::Video(path.to_path_buf())),
        Some("jpg" | "jpeg" | "png" | "gif") => Ok(MediaInput::Image(path.to_path_buf())),
        _ => Err("Unsupported Media type".into()),
    }
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
