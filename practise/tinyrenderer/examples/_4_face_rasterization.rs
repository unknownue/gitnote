
use tinyrenderer::tga::{TgaImage, TgaFormat};
use tinyrenderer::{Vec3f, Vec4f, Vec2f};
use tinyrenderer::rasterization::ZbufferEx;
use tinyrenderer::mesh::ObjMesh;
use tinyrenderer::rasterization::barycentric_rasterization_diffuse;

const OUTPUT_PATH: &'static str = "output.tga";


fn main() -> std::io::Result<()> {

    let mut image = TgaImage::new(800, 800, TgaFormat::RGB);
    let mut mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;
    mesh.load_diffuse_map("./assets/african_head/african_head_diffuse.tga")?;

    let light_dir = Vec3f::new(0.0, 0.0, -1.0);
    let camera_pos = Vec3f::new(0.0, 0.0, 1.5);

    let mut z_buffer = ZbufferEx { buffer: [std::f32::MIN; 800 * 800], width: 800 };
    let projection_matrix: vek::Mat4<f32> = vek::Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -1.0 / camera_pos.z, 1.0
    );

    for face in mesh.faces.iter() {
        let world_to_camera = |world: &Vec3f| -> Vec4f {
            let argument_world = Vec4f::new(world.x, world.y, world.z, 1.0);
            projection_matrix * argument_world
        };

        let camera_to_screen = |coord: Vec4f| -> Vec3f {
            let coord = coord / coord.w;
            Vec3f::new((coord.x + 1.0) * 400.0 + 0.5, (coord.y + 1.0) * 400.0 + 0.5, coord.z)
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
