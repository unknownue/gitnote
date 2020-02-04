
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
pub type Mat3f = vek::Mat3<f32>;


pub fn veci2f(v: Vec3i) -> Vec3f {
    Vec3f::new(v[0] as f32, v[1] as f32, v[2] as f32)
}
pub fn vecf2i(v: Vec3f) -> Vec3i {
    Vec3i::new(v[0] as i32, v[1] as i32, v[2] as i32)
}


pub trait Mat3Ext {
    fn set_column(&mut self, column_idx: usize, v: Vec3f);
    fn get_column(&self, column_idx: usize) -> Vec3f;
    fn from_row_vecs(vecs: [Vec3f; 3]) -> Self;
    fn from_columns_vecs(vecs: [Vec3f; 3]) -> Self;
    fn inverted(&self) -> Self;
}

impl Mat3Ext for Mat3f {
    fn set_column(&mut self, column_idx: usize, v: Vec3f) {
        self[(0, column_idx)] = v.x;
        self[(1, column_idx)] = v.y;
        self[(2, column_idx)] = v.z;
    }

    fn get_column(&self, column_idx: usize) -> Vec3f {
        Vec3f::new(self[(0, column_idx)], self[(1, column_idx)], self[(2, column_idx)])
    }

    fn from_row_vecs(vecs: [Vec3f; 3]) -> Mat3f {
        Mat3f::new(
            vecs[0].x, vecs[0].y, vecs[0].z,
            vecs[1].x, vecs[1].y, vecs[1].z,
            vecs[2].x, vecs[2].y, vecs[2].z,
        )
    }

    fn from_columns_vecs(vecs: [Vec3f; 3]) -> Mat3f {
        Mat3f::new(
            vecs[0].x, vecs[1].x, vecs[2].x,
            vecs[0].y, vecs[1].y, vecs[2].y,
            vecs[0].z, vecs[1].z, vecs[2].z,
        )
    }

    fn inverted(&self) -> Mat3f {
        // from https://stackoverflow.com/questions/983999/simple-3x3-matrix-inverse-code-c#answer-18504573
        let m = self;
        // computes the inverse of a matrix m
        let det: f32 = m.determinant();
        // let det = m[(0, 0)] * (m[(1, 1)] * m[(2, 2)] - m[(2, 1)] * m[(1, 2)]) -
        //     m[(0, 1)] * (m[(1, 0)] * m[(2, 2)] - m[(1, 2)] * m[(2, 0)]) +
        //     m[(0, 2)] * (m[(1, 0)] * m[(2, 1)] - m[(1, 1)] * m[(2, 0)]);
        let invdet = 1.0 / det;
        
        let result = Mat3f::new(
            m[(1, 1)] * m[(2, 2)] - m[(2, 1)] * m[(1, 2)],
            m[(0, 2)] * m[(2, 1)] - m[(0, 1)] * m[(2, 2)],
            m[(0, 1)] * m[(1, 2)] - m[(0, 2)] * m[(1, 1)],

            m[(1, 2)] * m[(2, 0)] - m[(1, 0)] * m[(2, 2)],
            m[(0, 0)] * m[(2, 2)] - m[(0, 2)] * m[(2, 0)],
            m[(1, 0)] * m[(0, 2)] - m[(0, 0)] * m[(1, 2)],

            m[(1, 0)] * m[(2, 1)] - m[(2, 0)] * m[(1, 1)],
            m[(2, 0)] * m[(0, 1)] - m[(0, 0)] * m[(2, 1)],
            m[(0, 0)] * m[(1, 1)] - m[(1, 0)] * m[(0, 1)],
        );
        result.map(|x| x * invdet)
    }
}
