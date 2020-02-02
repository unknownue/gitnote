
use tinyrenderer::tga::{TgaImage, TgaFormat};
use tinyrenderer::{Vec3f, Vec4f};
use tinyrenderer::rasterization::ZbufferEx;
use tinyrenderer::mesh::ObjMesh;
use tinyrenderer::rasterization::line_sweeping_gouraud_shading;
use tinyrenderer::camera::{lookat, viewport, projection};
use tinyrenderer::vecf2i;

const OUTPUT_PATH: &'static str = "output.tga";


fn main() -> std::io::Result<()> {

    let mut image = TgaImage::new(800, 800, TgaFormat::RGB);
    let mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;
    let mut z_buffer = ZbufferEx { buffer: [std::f32::MIN; 800 * 800], width: 800 };

    let light_dir = Vec3f::new(1.0, -1.0, 1.0).normalized();
    let eye_pos = Vec3f::new(1.0, 0.5, 5.0);
    let center = Vec3f::zero();

    let model_view = lookat(eye_pos, center, Vec3f::unit_y());
    let proj: vek::Mat4<f32> = projection((eye_pos - center).magnitude());
    let view_port = viewport(100, 100, 800 * 3 / 4, 800 * 3 / 4, 255);
    let affine_matrix = view_port * proj * model_view;

    for face in mesh.faces.iter() {

        let world_coords = [
            mesh.vertices[face[0]].position,
            mesh.vertices[face[1]].position,
            mesh.vertices[face[2]].position,
        ];

        let screen_coords = [
            vecf2i((affine_matrix * Vec4f::from_point(world_coords[0])).xyz()),
            vecf2i((affine_matrix * Vec4f::from_point(world_coords[1])).xyz()),
            vecf2i((affine_matrix * Vec4f::from_point(world_coords[2])).xyz()),
        ];

        let intensity = [
            Vec3f::dot(mesh.vertices[face[0]].normal, light_dir),
            Vec3f::dot(mesh.vertices[face[1]].normal, light_dir),
            Vec3f::dot(mesh.vertices[face[2]].normal, light_dir),
        ];

        line_sweeping_gouraud_shading(&mut image, &mut z_buffer, screen_coords[0], screen_coords[1], screen_coords[2], intensity[0], intensity[1], intensity[2]);
    }

    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}
