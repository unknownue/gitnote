
use tinyrenderer::tga::{TgaImage, TgaFormat, TgaColor};
use tinyrenderer::{Vec3f, Vec4f, Mat4f, Vec2f};
use tinyrenderer::rasterization::ZbufferEx;
use tinyrenderer::mesh::ObjMesh;
use tinyrenderer::rasterization::triangle;
use tinyrenderer::camera::{lookat, viewport, projection};
use tinyrenderer::shader::IShader;

const OUTPUT_PATH: &'static str = "output.tga";
const WIDTH : i32 = 800;
const HEIGHT: i32 = 800;
const LIGHT_DIR    : Vec3f = Vec3f::new(1.0, 1.0, 1.0);
const EYE_POSITION : Vec3f = Vec3f::new(1.0, 1.0, 3.0);
const CENTER       : Vec3f = Vec3f::new(0.0, 0.0, 0.0);
const UP           : Vec3f = Vec3f::new(0.0, 1.0, 0.0);

// --------------------------------------------------------------------------------------
struct GroundShader {
    mesh: ObjMesh,
    varying_intensity: Vec3f,
    affine_transform: Mat4f,
}

impl IShader for GroundShader {

    fn vertex(&mut self, vertex_idx: usize, nthvert: usize) -> Vec4f {
        let vertex = &self.mesh.vertices[vertex_idx];

        // get diffuse lighting intensity
        self.varying_intensity[nthvert] = f32::max(0.0, Vec3f::dot(vertex.normal, LIGHT_DIR.normalized()));

        let gl_vertex = Vec4f::from_point(vertex.position); // set last coordinate to 1
        self.affine_transform * gl_vertex // transform it to screen coordinates
    }

    fn fragment(&self, barycentric: Vec3f) -> Option<TgaColor> {
        let intensity: f32 = Vec3f::dot(self.varying_intensity, barycentric);  // interpolate intensity for the current pixel
        let color = TgaColor::from_rgb(255, 255, 255) * intensity;
        Some(color)
    }
}


struct ToonShader {
    mesh: ObjMesh,
    varying_intensity: Vec3f,
    affine_transform: Mat4f,
}

impl IShader for ToonShader {

    fn vertex(&mut self, vertex_idx: usize, nthvert: usize) -> Vec4f {
        let vertex = &self.mesh.vertices[vertex_idx];

        self.varying_intensity[nthvert] = f32::max(0.0, Vec3f::dot(vertex.normal, LIGHT_DIR.normalized()));
        let gl_vertex = Vec4f::from_point(vertex.position);
        self.affine_transform * gl_vertex
    }

    fn fragment(&self, barycentric: Vec3f) -> Option<TgaColor> {
        let mut intensity: f32 = Vec3f::dot(self.varying_intensity, barycentric);
        if intensity > 0.85 { intensity = 1.0; }
        else if intensity > 0.60 { intensity = 0.80; }
        else if intensity > 0.45 { intensity = 0.60; }
        else if intensity > 0.30 { intensity = 0.45; }
        else if intensity > 0.15 { intensity = 0.30; }
        else { intensity = 0.0; }

        let color = TgaColor::from_rgb(255, 155, 0) * intensity;
        Some(color)
    }
}

// https://github.com/ssloy/tinyrenderer/wiki/Lesson-6-Shaders-for-the-software-renderer#my-implementation-of-shaders-shown-on-gouraud-shading
#[allow(unused)]
fn ground_shading(image: &mut TgaImage, affine_transform: Mat4f, mut z_buffer: ZbufferEx) -> std::io::Result<()> {

    let mesh = ObjMesh::load_mesh("./assets/diablo3_pose/diablo3_pose.obj")?;
    let faces = mesh.faces.clone();

    let mut shader = GroundShader { // or GroundShader
        mesh,
        varying_intensity: Vec3f::zero(),
        affine_transform,
    };
    for face in faces {

        let screen_coords = [
            shader.vertex(face[0], 0),
            shader.vertex(face[1], 1),
            shader.vertex(face[2], 2),
        ];

        triangle(image, &shader, &mut z_buffer, screen_coords);
    }
    Ok(())
}
// --------------------------------------------------------------------------------------


// --------------------------------------------------------------------------------------
struct PhongShader {
    mesh: ObjMesh,
    varying_intensity: Vec3f,
    varying_uv: [Vec2f; 3],
    affine_transform: Mat4f,
}

impl IShader for PhongShader {

    fn vertex(&mut self, vertex_idx: usize, nthvert: usize) -> Vec4f {
        let vertex = &self.mesh.vertices[vertex_idx];

        self.varying_intensity[nthvert] = f32::max(0.0, Vec3f::dot(vertex.normal, LIGHT_DIR.normalized()));
        self.varying_uv[nthvert] = vertex.uv;

        let gl_vertex = Vec4f::from_point(vertex.position);
        self.affine_transform * gl_vertex
    }

    fn fragment(&self, barycentric: Vec3f) -> Option<TgaColor> {

        fn sample_barycentric_uv(uvs: &[Vec2f; 3], bc: Vec3f) -> Vec2f {
            Vec2f::new(
                uvs[0].x * bc.x + uvs[1].x * bc.y + uvs[2].x * bc.z,
                uvs[0].y * bc.x + uvs[1].y * bc.y + uvs[2].y * bc.z,
            )
        }

        let intensity: f32 = Vec3f::dot(self.varying_intensity, barycentric);
        // interpolate uv for the current pixel
        let uv = sample_barycentric_uv(&self.varying_uv, barycentric);
        let color = self.mesh.diffuse_map.sample_diffuse(uv) * intensity;
        Some(color)
    }
}

// https://github.com/ssloy/tinyrenderer/wiki/Lesson-6-Shaders-for-the-software-renderer#textures
#[allow(unused)]
fn textures(image: &mut TgaImage, affine_transform: Mat4f, mut z_buffer: ZbufferEx) -> std::io::Result<()> {

    let mut mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;
    let faces = mesh.faces.clone();
    mesh.load_diffuse_map("./assets/african_head/african_head_diffuse.tga")?;

    let mut shader = PhongShader { // or GroundShader
        mesh,
        varying_intensity: Vec3f::zero(),
        varying_uv: [Vec2f::zero(); 3],
        affine_transform,
    };

    for face in faces {
        let screen_coords = [
            shader.vertex(face[0], 0),
            shader.vertex(face[1], 1),
            shader.vertex(face[2], 2),
        ];
        triangle(image, &shader, &mut z_buffer, screen_coords);
    }
    Ok(())
}
// --------------------------------------------------------------------------------------


// --------------------------------------------------------------------------------------
struct NormalMappingShader {
    mesh: ObjMesh,
    varying_uv: [Vec2f; 3],

    uniform_m  : Mat4f, // Projection*ModelView
    uniform_mit: Mat4f, // (Projection*ModelView).invert_transpose()
    affine_transform: Mat4f,
}

impl IShader for NormalMappingShader {

    fn vertex(&mut self, vertex_idx: usize, nthvert: usize) -> Vec4f {
        let vertex = &self.mesh.vertices[vertex_idx];

        self.varying_uv[nthvert] = vertex.uv;
        let gl_vertex = Vec4f::from_point(vertex.position);
        self.affine_transform * gl_vertex // transform it to screen coordinates
    }

    fn fragment(&self, barycentric: Vec3f) -> Option<TgaColor> {

        fn sample_barycentric_uv(uvs: &[Vec2f; 3], bc: Vec3f) -> Vec2f {
            Vec2f::new(
                uvs[0].x * bc.x + uvs[1].x * bc.y + uvs[2].x * bc.z,
                uvs[0].y * bc.x + uvs[1].y * bc.y + uvs[2].y * bc.z,
            )
        }

        let uv = sample_barycentric_uv(&self.varying_uv, barycentric);
        let n = (self.uniform_mit * Vec4f::from_point(self.mesh.normal_map.sample_normal(uv))).normalized().xyz();
        let l = (self.uniform_m   * Vec4f::from_point(LIGHT_DIR)).normalized().xyz();

        let intensity: f32 = f32::max(0.0, Vec3f::dot(n, l));
        let color = self.mesh.diffuse_map.sample_diffuse(uv) * intensity;
        Some(color)
    }
}

// https://github.com/ssloy/tinyrenderer/wiki/Lesson-6-Shaders-for-the-software-renderer#normalmapping
#[allow(unused)]
fn normal_mapping(image: &mut TgaImage, projection: Mat4f, model_view: Mat4f, viewport: Mat4f, mut z_buffer: ZbufferEx) -> std::io::Result<()> {

    let mut mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;
    let faces = mesh.faces.clone();
    mesh.load_diffuse_map("./assets/african_head/african_head_diffuse.tga")?;
    mesh.load_normal_map("./assets/african_head/african_head_nm_tangent.tga")?;

    let mut shader = NormalMappingShader {
        mesh,
        varying_uv: [Vec2f::zero(); 3],
        uniform_m       : projection * model_view,
        uniform_mit     : (projection * model_view).transposed(),
        affine_transform: viewport * projection * model_view,
    };

    for face in faces {
        let screen_coords = [
            shader.vertex(face[0], 0),
            shader.vertex(face[1], 1),
            shader.vertex(face[2], 2),
        ];
        triangle(image, &shader, &mut z_buffer, screen_coords);
    }
    Ok(())
}
// --------------------------------------------------------------------------------------


fn main() -> std::io::Result<()> {

    let mut image = TgaImage::new(WIDTH, HEIGHT, TgaFormat::RGB);
    let z_buffer = ZbufferEx { buffer: [std::f32::MIN; (WIDTH * WIDTH) as usize], width: WIDTH as usize };

    let model_view: vek::Mat4<f32> = lookat(EYE_POSITION, CENTER, UP);
    let projection: vek::Mat4<f32> = projection((EYE_POSITION - CENTER).magnitude());
    let view_port : vek::Mat4<f32> = viewport(WIDTH / 8, HEIGHT / 8, WIDTH as u32 * 3 / 4, HEIGHT as u32 * 3 / 4, 255);
    let affine_matrix = view_port * projection * model_view;

    // ground_shading(&mut image, affine_matrix, z_buffer)?;
    // textures(&mut image, affine_matrix, z_buffer)?;
    normal_mapping(&mut image, projection, model_view, view_port, z_buffer)?;

    image.flip_vertically(); // place the origin in the bottom left corner of the image
    image.write_tga_file(OUTPUT_PATH, true)
}
