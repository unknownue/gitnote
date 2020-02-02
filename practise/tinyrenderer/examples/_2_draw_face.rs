
use tinyrenderer::tga::{TgaImage, TgaFormat, TgaColor};
use tinyrenderer::mesh::ObjMesh;
use tinyrenderer::bresenham::line_segment_v3 as draw_line;

const OUTPUT_PATH: &'static str = "output.tga";
const WHITE: TgaColor = TgaColor::from_rgb(255, 255, 255);


fn main() -> std::io::Result<()> {

    let mut image = TgaImage::new(800, 800, TgaFormat::RGB);
    let mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;

    for face in mesh.faces.iter() {
        for j in 0..3 {
            let v0 = &mesh.vertices[face[j]];
            let v1 = &mesh.vertices[face[(j + 1) % 3]];

            // convert [-1, 1] to [0, 800.0]
            let x0 = ((v0.position[0] + 1.0) * 400.0) as i32;
            let y0 = ((v0.position[1] + 1.0) * 400.0) as i32;
            let x1 = ((v1.position[0] + 1.0) * 400.0) as i32;
            let y1 = ((v1.position[1] + 1.0) * 400.0) as i32;

            draw_line(&mut image, x0, y0, x1, y1, &WHITE)
        }
    }

    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}
