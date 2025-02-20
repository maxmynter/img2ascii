use clap::{Parser, ValueEnum};

const DEFAULT_DIMENSION: u32 = 100;

#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,

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

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorScheme {
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
