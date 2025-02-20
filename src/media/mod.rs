mod image;
mod video;

use crate::config::{Cli, MediaSource};
use ffmpeg_next as ffmpeg;
pub use image::*;
use std::time::Duration;
use std::{error::Error, path::PathBuf};
pub use video::*;

struct VideoContext {
    input: ffmpeg::format::context::Input,
    decoder: ffmpeg::decoder::Video,
    frame_duration: Duration,
}

pub enum MediaInput {
    Image(PathBuf),
    Video(PathBuf),
    WebCam(u32),
}

impl MediaInput {
    pub fn from_cli(args: &Cli) -> Result<MediaInput, Box<dyn Error>> {
        match args.source {
            MediaSource::File => {
                // TODO: Make this validation in the CLI struct. Here it's hacky :/
                if args.file.as_os_str().is_empty() {
                    return Err("File path is required when source is 'file'".into());
                }
                match args.file.extension().and_then(|ext| ext.to_str()) {
                    Some("mp4" | "avi" | "mov" | "mkv") => Ok(MediaInput::Video(args.file.clone())),
                    Some("jpg" | "jpeg" | "png" | "gif") => {
                        Ok(MediaInput::Image(args.file.clone()))
                    }
                    _ => Err("Unsupported Media type".into()),
                }
            }
            MediaSource::WebCam => Ok(MediaInput::WebCam(args.device)),
        }
    }
}
