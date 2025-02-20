use clap::{Parser, ValueEnum};

const DEFAULT_DIMENSION: u32 = 100;

#[derive(ValueEnum, Parser, Debug, Clone)]
pub enum MediaSource {
    File,
    WebCam,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorScheme {
    Original,
    BlackAndWhite,
    Pastel,
}

macro_rules! impl_value_enum_display {
    ($type: ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.to_possible_value()
                    .expect("no values skipped")
                    .get_name()
                    .fmt(f)
            }
        }
    };
}

impl_value_enum_display!(ColorScheme);
impl_value_enum_display!(MediaSource);

#[derive(Parser)]
pub struct Cli {
    #[arg(short='s', long, default_value_t = MediaSource::File)]
    pub source: MediaSource,

    #[arg(short = 'd', long, default_value_t = 0)]
    pub device: u32,

    #[arg(short = 'f', long)]
    pub file: std::path::PathBuf,

    #[arg(short = 'x', long, default_value_t = DEFAULT_DIMENSION)]
    pub width: u32,

    #[arg(short = 'y', long, default_value_t = DEFAULT_DIMENSION)]
    pub height: u32,

    #[arg(short = 'c', long, default_value_t = ColorScheme::Original)]
    pub color_scheme: ColorScheme,

    #[arg(short= 'g', long, default_value_t = 1.0,value_parser=validate_granularity )]
    pub granularity: f32,
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
