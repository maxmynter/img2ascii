use crate::ascii::generate_ascii_art;
use crate::config::Cli;
use std::error::Error;

pub fn generate_ascii_image(args: Cli) -> Result<String, Box<dyn Error>> {
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
