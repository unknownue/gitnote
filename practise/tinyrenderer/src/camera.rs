
use crate::{Vec3f, Mat4f};

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Mat4f {

    // https://github.com/ssloy/tinyrenderer/wiki/Lesson-5-Moving-the-camera#let-us-create-our-own-glulookat
    // See also https://www.jianshu.com/p/64b4c887c439
    let z = (eye - center).normalized();
    let x = Vec3f::cross(up, z).normalized();
    let y = Vec3f::cross(z, x).normalized();

    let mut minv = Mat4f::identity();
    let mut translation = Mat4f::identity();

    for i in 0..3 {
        minv[(0, i)] = x[i];
        minv[(1, i)] = y[i];
        minv[(2, i)] = z[i];
        translation[(i, 3)] = -center[i];
    }

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
