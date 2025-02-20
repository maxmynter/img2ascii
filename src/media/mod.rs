mod image;
mod video;

use ffmpeg_next as ffmpeg;
pub use image::*;
use std::time::Duration;
pub use video::*;
struct VideoContext {
    input: ffmpeg::format::context::Input,
    decoder: ffmpeg::decoder::Video,
    frame_duration: Duration,
}
