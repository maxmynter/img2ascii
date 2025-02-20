mod ascii;
mod config;
mod media;

use clap::Parser;
use config::Cli;
use media::{generate_ascii_image, process_video, MediaInput};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let input = MediaInput::from_cli(&args)?;

    match input {
        MediaInput::Image(_path) => {
            let ascii_art = generate_ascii_image(args)?;
            print!("{}", ascii_art);
        }
        MediaInput::Video(_path) => {
            process_video(args)?;
        }
        MediaInput::WebCam(_idx) => todo!("Webcam not yet implemented."),
    }

    Ok(())
}
