//!
//! https://github.com/ssloy/tinyrenderer/wiki/Lesson-7-Shadow-mapping
//!

use tinyrenderer::tga::{TgaImage, TgaFormat, TgaColor};
use tinyrenderer::{Vec3f, Vec4f, Mat4f, Vec2f, Mat3f};
use tinyrenderer::rasterization::{ZbufferEx, ZBuffer};
use tinyrenderer::mesh::ObjMesh;
use tinyrenderer::rasterization::triangle;
use tinyrenderer::camera::{lookat, viewport, projection, sample_barycentric_uv};
use tinyrenderer::shader::IShader;
use tinyrenderer::Mat3Ext;

const OUTPUT_PATH: &'static str = "output.tga";
const WIDTH : i32 = 800;
const HEIGHT: i32 = 800;
const LIGHT_DIR    : Vec3f = Vec3f::new(1.0, 1.0, 0.0);
const EYE_POSITION : Vec3f = Vec3f::new(1.0, 1.0, 4.0);
const CENTER       : Vec3f = Vec3f::new(0.0, 0.0, 0.0);
const UP           : Vec3f = Vec3f::new(0.0, 1.0, 0.0);
const DEPTH: f32 = 2000.0;

type ShadowBuffer = ZbufferEx;


// --------------------------------------------------------------------------------------
struct DepthShader {
    mesh: ObjMesh,
    varying_tri: Mat3f,
    affine_transform: Mat4f,
}

impl IShader for DepthShader {

    fn vertex(&mut self, vertex_idx: usize, nthvert: usize) -> Vec4f {
        let vertex = &self.mesh.vertices[vertex_idx];
        let gl_vertex = self.affine_transform * Vec4f::from_point(vertex.position);
        self.varying_tri.set_column(nthvert, gl_vertex.homogenized().xyz());
        gl_vertex
    }

    fn fragment(&self, barycentric: Vec3f) -> Option<TgaColor> {
        let p = self.varying_tri * barycentric;
        let color = TgaColor::from_rgb(255, 255, 255) * (p.z / DEPTH);
        Some(color)
    }
}

fn render_shadow(shadow: &mut ShadowBuffer) -> std::io::Result<TgaImage> {

    let model_view: Mat4f = lookat(LIGHT_DIR.normalized(), CENTER, UP);
    let projection: Mat4f = projection(0.0);
    let view_port : Mat4f = viewport(WIDTH / 8, HEIGHT / 8, WIDTH as u32 * 3 / 4, HEIGHT as u32 * 3 / 4, DEPTH);

    let mesh = ObjMesh::load_mesh("./assets/diablo3_pose/diablo3_pose.obj")?;
    let faces = mesh.faces.clone();
    let mut depth_image = TgaImage::new(WIDTH, HEIGHT, TgaFormat::RGB);

    let mut shader = DepthShader {
        mesh,
        varying_tri: Mat3f::identity(),
        affine_transform: view_port * projection * model_view,
    };

    for face in faces {
        let screen_coords = [
            shader.vertex(face[0], 0),
            shader.vertex(face[1], 1),
            shader.vertex(face[2], 2),
        ];
        triangle(&mut depth_image, &shader, shadow, screen_coords, DEPTH);
    }
    Ok(depth_image)
}
// --------------------------------------------------------------------------------------


// --------------------------------------------------------------------------------------
struct ShadowShader {
    mesh: ObjMesh,
    shadow_buffer: ShadowBuffer,
    varying_tri: Mat3f,     // triangle coordinates before Viewport transform, written by vertex shader, read by fragment shader
    varying_uv: [Vec2f; 3], // triangle uv coordinates, written by the vertex shader, read by the fragment shader

    uniform_m  : Mat4f,      // Projection * ModelView
    uniform_mit: Mat4f,      // (Projection * ModelView).invert_transpose()
    uniform_m_shadow: Mat4f, // transform framebuffer screen coordinates to shadowbuffer screen coordinates
    affine_transform: Mat4f,
}

impl IShader for ShadowShader {

    fn vertex(&mut self, vertex_idx: usize, nthvert: usize) -> Vec4f {
        let vertex = &self.mesh.vertices[vertex_idx];
        let gl_vertex = self.affine_transform * Vec4f::from_point(vertex.position);
        self.varying_tri.set_column(nthvert, gl_vertex.homogenized().xyz());
        gl_vertex
    }

    fn fragment(&self, barycentric: Vec3f) -> Option<TgaColor> {

        // corresponding point in the shadow buffer
        let sb_p: Vec3f = (self.uniform_m_shadow * Vec4f::from_point(self.varying_tri * barycentric)).homogenized().xyz();
        // magic coeff to avoid z-fighting
//        let shadow = if self.shadow_buffer.get(sb_p.x as usize, sb_p.y as usize) < sb_p.z {
//            0.3 + 0.7
//        } else {
//            0.3
//        };
        let shadow = 0.3;

        let uv = sample_barycentric_uv(&self.varying_uv, barycentric);
        let n = (self.uniform_mit * Vec4f::from_point(self.mesh.sample_normal(uv))).normalized().xyz(); // normal
        let l = (self.uniform_m   * Vec4f::from_point(LIGHT_DIR)).normalized().xyz(); // light vector
        let r = (2.0 * n * Vec3f::dot(n, l) - l).normalized(); // reflected light

        let specular = f32::max(r.z, 0.0).powf(self.mesh.sample_specular(uv) / 10.0); // 10.0 is magic number
        let diff = f32::max(0.0, Vec3f::dot(n, l));

        let mut color: TgaColor = self.mesh.sample_diffuse(uv);
        for i in 0..3 {
            color[i] = (20.0 + color[i] as f32 * shadow * (1.2 * diff + 0.6 * specular)).min(255.0) as u8;
        }
        Some(color)
    }
}

fn framebuffer(shadow: ShadowBuffer) -> std::io::Result<TgaImage> {

    let mut image = TgaImage::new(WIDTH, HEIGHT, TgaFormat::RGB);
    let mut z_buffer = ZbufferEx { buffer: vec![std::f32::MIN; (WIDTH * WIDTH) as usize], width: WIDTH as usize };

    let model_view: vek::Mat4<f32> = lookat(EYE_POSITION, CENTER, UP);
    let view_port : vek::Mat4<f32> = viewport(WIDTH / 8, HEIGHT / 8, WIDTH as u32 * 3 / 4, HEIGHT as u32 * 3 / 4, DEPTH);
    let projection: vek::Mat4<f32> = projection(-1.0 / (EYE_POSITION - CENTER).magnitude());

    let mut mesh = ObjMesh::load_mesh("./assets/diablo3_pose/diablo3_pose.obj")?;
    let faces = mesh.faces.clone();
    mesh.load_diffuse_map("./assets/diablo3_pose/diablo3_pose_diffuse.tga")?;
    mesh.load_normal_map("./assets/diablo3_pose/diablo3_pose_nm.tga")?;
    mesh.load_specular_map("./assets/diablo3_pose/diablo3_posed_spec.tga")?;

    let mut shader = ShadowShader {
        mesh,
        shadow_buffer: shadow,
        varying_tri: Mat3f::identity(),
        varying_uv: [Vec2f::zero(); 3],

        uniform_m       : projection * model_view,
        uniform_mit     : (projection * model_view).inverted().transposed(),
        uniform_m_shadow: view_port * lookat(LIGHT_DIR.normalized(), CENTER, UP),
        affine_transform: view_port * projection * model_view,
    };

    for face in faces {
        let screen_coords = [
            shader.vertex(face[0], 0),
            shader.vertex(face[1], 1),
            shader.vertex(face[2], 2),
        ];
        triangle(&mut image, &shader, &mut z_buffer, screen_coords, DEPTH);
    }
    Ok(image)
}
// --------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {

    // rendering the shadow buffer
    let mut shadow = ShadowBuffer { buffer: vec![std::f32::MIN; (WIDTH * WIDTH) as usize], width: WIDTH as usize };

    let mut depth_image = render_shadow(&mut shadow)?;
    let mut image = framebuffer(shadow)?;

    depth_image.flip_vertically();
    depth_image.write_tga_file("depth.tga", true)?;

    image.flip_vertically();
    image.write_tga_file(OUTPUT_PATH, true)
}
