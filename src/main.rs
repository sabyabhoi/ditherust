use color_eyre::eyre::Result;
use clap::Parser;

use ditherust::process_image;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser)]
    /// Path of image to dither 
    filename: String,
    
    #[clap(short, long, value_parser)]
    /// Path to theme
    theme: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    process_image(&args.filename.as_str(), &args.theme.as_str())?;

    Ok(())
}
