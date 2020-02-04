
use crate::{Vec3f, Vec2f, Mat4f};

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Mat4f {

    // https://github.com/ssloy/tinyrenderer/wiki/Lesson-5-Moving-the-camera#let-us-create-our-own-glulookat
    // See also https://www.jianshu.com/p/64b4c887c439
    let z = (eye - center).normalized();
    let x = Vec3f::cross(up, z).normalized();
    let y = Vec3f::cross(z, x).normalized();

    let minv = Mat4f::new(
        x[0], x[1], x[2], 0.0,
        y[0], y[1], y[2], 0.0,
        z[0], z[1], z[2], 0.0,
         0.0,  0.0,  0.0, 1.0,
    );
    let translation = Mat4f::new(
        1.0, 0.0, 0.0, -center[0],
        0.0, 1.0, 0.0, -center[1],
        0.0, 0.0, 1.0, -center[2],
        0.0, 0.0, 0.0, 1.0,
    );

    minv * translation
}

pub fn viewport(x: i32, y: i32, w: u32, h: u32, depth: u32) -> Mat4f {

    let (x, y, w, h, d) = (x as f32, y as f32, w as f32, h as f32, depth as f32);
    Mat4f::new(
        w / 2.0,     0.0,     0.0, x + w / 2.0,
            0.0, h / 2.0,     0.0, y + h / 2.0,
            0.0,     0.0, d / 2.0,     d / 2.0,
            0.0,     0.0,     0.0,         1.0,
    )
}

pub fn projection(coeff: f32) -> Mat4f {
    vek::Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -1.0 / coeff, 1.0,
    )
}


pub fn sample_barycentric_uv(uvs: &[Vec2f; 3], bc: Vec3f) -> Vec2f {
    Vec2f::new(
        uvs[0].x * bc.x + uvs[1].x * bc.y + uvs[2].x * bc.z,
        uvs[0].y * bc.x + uvs[1].y * bc.y + uvs[2].y * bc.z,
    )
}
