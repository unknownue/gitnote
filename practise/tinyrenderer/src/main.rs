
use tinyrenderer::tga::{TgaImage, TgaFormat, TgaColor};
use tinyrenderer::{Vec2i, Vec3f, Vec4f, Vec2f};
use tinyrenderer::bresenham::line_segment_v3 as draw_line;


const OUTPUT_PATH: &'static str = "output.tga";
const RED  : TgaColor = TgaColor::from_rgb(0,0,255);
const WHITE: TgaColor = TgaColor::from_rgb(255, 255, 255);


fn main() -> std::io::Result<()> {

    // test_tga()
    // test_draw_line()
    // test_draw_face()
    // test_triangle_rasterization()
    test_face_rasterization()
}

fn test_face_rasterization() -> std::io::Result<()> {

    use tinyrenderer::mesh::ObjMesh;
    use tinyrenderer::rasterization::barycentric_rasterization_diffuse;
    use tinyrenderer::rasterization::ZBuffer;

    let mut image = TgaImage::new(800, 800, TgaFormat::RGB);
    let mut mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;
    mesh.load_diffuse_map("./assets/african_head/african_head_diffuse.tga")?;
    let light_dir = Vec3f::new(0.0, 0.0, -1.0);
    let camera_pos = Vec3f::new(0.0, 0.0, 3.0);

    struct ZbufferEx { buffer: [f32; 800 * 800], width: usize }
    impl ZBuffer for ZbufferEx {
        fn get(&self, x: usize, y: usize) -> f32 { self.buffer[x + y * self.width] }
        fn set(&mut self, x: usize, y: usize, v: f32) { self.buffer[x + y * self.width] = v; }
    }

    let mut z_buffer = ZbufferEx { buffer: [std::f32::MIN; 800 * 800], width: 800 };
    let projection_matrix: vek::Mat4<f32> = vek::Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -1.0 / camera_pos.z, 1.0
    );
    let viewport = vek::Mat4::new(
        400.0, 0.0, 0.0, 800.0 / 8.0 + 800.0 / 2.0,
        0.0, 400.0, 0.0, 800.0 / 8.0 + 800.0 / 2.0,
        0.0, 0.0, 255.0 / 2.0, 255.0 / 2.0,
        0.0, 0.0, 0.0, 1.0
    );

    for face in mesh.faces.iter() {
        let world_to_camera = |world: &Vec3f| -> Vec4f {
            let argument_world = Vec4f::new(world.x, world.y, world.z, 1.0);
            let coord_in_camera: Vec4f = projection_matrix * argument_world;
            coord_in_camera
        };

        let camera_to_screen = |coord: Vec4f| -> Vec3f {
//            Vec3f::new(((coord.x + 1.0) * 400.0 + 0.5).floor(), ((coord.y + 1.0) * 400.0 + 0.5).floor(), coord.z)
            let screen_coord = viewport * coord;
            Vec3f::new(screen_coord.x / screen_coord.w, screen_coord.y / screen_coord.w, screen_coord.z / screen_coord.w)
        };

        let world_coords = [
            mesh.vertices[face[0]].position,
            mesh.vertices[face[1]].position,
            mesh.vertices[face[2]].position,
        ];

        let screen_coords = [
            camera_to_screen(world_to_camera(&world_coords[0])),
            camera_to_screen(world_to_camera(&world_coords[1])),
            camera_to_screen(world_to_camera(&world_coords[2])),
        ];

        let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]).normalized();
        let intensity = n.dot(light_dir);
        if intensity > 0.0 {
            let uv_multiplier = Vec2f::new(mesh.diffuse_map.width as f32, mesh.diffuse_map.height as f32);
            let uvs = [
                mesh.vertices[face[0]].uv * uv_multiplier,
                mesh.vertices[face[1]].uv * uv_multiplier,
                mesh.vertices[face[2]].uv * uv_multiplier,
            ];

            barycentric_rasterization_diffuse(&mut image, &mut z_buffer, screen_coords, uvs, &mesh.diffuse_map, intensity)?;
        }
    }

    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}

#[allow(unused)]
fn test_triangle_rasterization() -> std::io::Result<()> {

    use tinyrenderer::rasterization::barycentric_rasterization_v1;

    let mut image = TgaImage::new(200, 200, TgaFormat::RGB);

    let pts = [Vec2i::new(10, 10), Vec2i::new(100, 30), Vec2i::new(190, 160)];

    barycentric_rasterization_v1(&mut image, pts, &RED);

    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}

#[allow(unused)]
fn test_draw_face() -> std::io::Result<()> {

    use tinyrenderer::mesh::ObjMesh;

    let mut image = TgaImage::new(800, 800, TgaFormat::RGB);
    let mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;

    for face in mesh.faces.iter() {
        for j in 0..3 {
            let v0 = &mesh.vertices[face[j]];
            let v1 = &mesh.vertices[face[(j + 1) % 3]];

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

#[allow(unused)]
fn test_draw_line() -> std::io::Result<()> {

    let mut image = TgaImage::new(100, 100, TgaFormat::RGB);
    draw_line(&mut image, 13, 20, 80, 40, &WHITE);
    draw_line(&mut image, 20, 13, 40, 80, &RED);
    draw_line(&mut image, 80, 40, 13, 20, &RED);
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
