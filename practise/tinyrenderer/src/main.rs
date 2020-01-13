
use tinyrenderer::tga::TgaImage;
use tinyrenderer::tga::{TgaFormat, TgaColor};

const OUTPUT_PATH: &'static str = "output.tga";
const RED  : TgaColor = TgaColor::from_rgb(0,0,255);
const WHITE: TgaColor = TgaColor::from_rgb(255, 255, 255);


fn main() -> std::io::Result<()> {

    // test_tga()
    // test_draw_line()
    test_draw_face()
}

fn test_draw_face() -> std::io::Result<()> {

    use tinyrenderer::bresenham::line_segment_v3;
    use tinyrenderer::mesh::ObjMesh;

    let mut image = TgaImage::new(800, 800, TgaFormat::RGB);
    let mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;

    for face in mesh.faces.iter() {
        for j in 0..3 {
            let v0 = &mesh.vertices[face[j]];
            let v1 = &mesh.vertices[face[(j + 1) % 3]];

            let x0 = ((v0.position[0] + 1.0) * 400.0) as usize;
            let y0 = ((v0.position[1] + 1.0) * 400.0) as usize;
            let x1 = ((v1.position[0] + 1.0) * 400.0) as usize;
            let y1 = ((v1.position[1] + 1.0) * 400.0) as usize;

            line_segment_v3(&mut image, x0, y0, x1, y1, WHITE)?;
        }
    }

    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}

#[allow(unused)]
fn test_draw_line() -> std::io::Result<()> {

    use tinyrenderer::bresenham::line_segment_v3;

    let mut image = TgaImage::new(100, 100, TgaFormat::RGB);
    line_segment_v3(&mut image, 13, 20, 80, 40, WHITE)?;
    line_segment_v3(&mut image, 20, 13, 40, 80, RED)?;
    line_segment_v3(&mut image, 80, 40, 13, 20, RED)?;
    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}

#[allow(unused)]
fn test_tga() -> std::io::Result<()> {

    // let mut image = TgaImage::new(100, 100, TgaFormat::RGB);
    // image.set(52, 41, &RED)?;
    // image.write_tga_file(OUTPUT_PATH, false)?;

    let image = TgaImage::from_path("assets/grid.tga")?;
    image.write_tga_file(OUTPUT_PATH, false)?;

    Ok(())
}
