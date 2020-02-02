
pub mod tga;
pub mod bresenham;
pub mod mesh;
pub mod rasterization;
pub mod camera;
pub mod shader;

pub type Vec4f = vek::Vec4<f32>;
pub type Vec3f = vek::Vec3<f32>;
pub type Vec2f = vek::Vec2<f32>;
pub type Vec2i = vek::Vec2<i32>;
pub type Vec3i = vek::Vec3<i32>;
pub type Mat4f = vek::Mat4<f32>;


pub fn veci2f(v: Vec3i) -> Vec3f {
    Vec3f::new(v[0] as f32, v[1] as f32, v[2] as f32)
}
pub fn vecf2i(v: Vec3f) -> Vec3i {
    Vec3i::new(v[0] as i32, v[1] as i32, v[2] as i32)
}
