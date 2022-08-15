mod point3d;
mod theme;
use theme::Theme;

use std::path::Path;

use color_eyre::eyre::Result;
use image::imageops::colorops;

pub fn process_image(filename: &str, theme: &str) -> Result<()> {
    let mut img = image::open(&Path::new(filename))?.into_rgb8();

    let theme = Theme::read_from_file(&Path::new(theme))?;
    colorops::dither(&mut img, &theme);
    img.save("dithered.jpeg")?;

    Ok(())
}
