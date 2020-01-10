
mod tga;

use tga::{TgaImage, TgaFormat, TgaColor};

const OUTPUT_PATH: &'static str = "output.tga";

fn main() -> std::io::Result<()> {

    // const WRITE: TgaColor = TgaColor::from_rgba(255, 255, 255, 255);
    // const RED  : TgaColor = TgaColor::from_rgba(255,   0,   0, 255);

    // let mut image = TgaImage::new(100, 100, TgaFormat::RGB);
    // image.set(52, 41, &RED)?;
    // image.write_tga_file(OUTPUT_PATH, false)?;

    let image = TgaImage::from_path("grid.tga")?;
    image.write_tga_file(OUTPUT_PATH, false)?;

    Ok(())
}
