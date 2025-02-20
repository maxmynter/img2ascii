use super::VideoContext;
use crate::ascii::generate_ascii_art;
use crate::config::Cli;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::software::scaling::{self, Context};
use std::error::Error;
use std::io::{self, Write};
use std::time::{Duration, Instant};

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

    let mut buffer = Vec::new();
    let linesize = rgb_frame.stride(0);
    for y in 0..target_height {
        let start = y as usize * linesize;
        let end = start + (target_width as usize * 3);
        buffer.extend_from_slice(&rgb_frame.data(0)[start..end]);
    }

    Ok(image::DynamicImage::ImageRgb8(
        image::ImageBuffer::from_raw(target_width, target_height, buffer)
            .ok_or("failed to create image buffer")?,
    ))
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

pub fn process_video(args: Cli) -> Result<(), Box<dyn Error>> {
    let VideoContext {
        mut input,
        mut decoder,
        frame_duration,
    } = init_video(&args.file)?;

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
