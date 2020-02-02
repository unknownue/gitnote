
use crate::{Vec3f, Vec4f};
use crate::tga::TgaColor;

pub trait IShader {
    fn vertex(&mut self, vertex_idx: usize, nthvert: usize) -> Vec4f;
    fn fragment(&self, barycentric: Vec3f) -> Option<TgaColor>;
}
