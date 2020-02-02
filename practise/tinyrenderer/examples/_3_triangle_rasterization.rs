
use tinyrenderer::tga::{TgaImage, TgaFormat, TgaColor};
use tinyrenderer::Vec2i;

const OUTPUT_PATH: &'static str = "output.tga";
const RED  : TgaColor = TgaColor::from_rgb(0,0,255);


fn main() -> std::io::Result<()> {

    use tinyrenderer::rasterization::barycentric_rasterization_v1;

    let mut image = TgaImage::new(200, 200, TgaFormat::RGB);

    let pts = [Vec2i::new(10, 10), Vec2i::new(100, 30), Vec2i::new(190, 160)];

    barycentric_rasterization_v1(&mut image, pts, &RED);

    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}
