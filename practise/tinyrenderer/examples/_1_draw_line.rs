
use tinyrenderer::tga::{TgaImage, TgaFormat, TgaColor};
use tinyrenderer::bresenham::line_segment_v3 as draw_line;

const OUTPUT_PATH: &'static str = "output.tga";
const RED  : TgaColor = TgaColor::from_rgb(0,0,255);
const WHITE: TgaColor = TgaColor::from_rgb(255, 255, 255);

fn main() -> std::io::Result<()> {

    let mut image = TgaImage::new(100, 100, TgaFormat::RGB);
    draw_line(&mut image, 13, 20, 80, 40, &WHITE);
    draw_line(&mut image, 20, 13, 40, 80, &RED);
    draw_line(&mut image, 80, 40, 13, 20, &RED);
    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}
