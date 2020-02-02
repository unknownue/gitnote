
use crate::tga::{TgaColor, TgaImage};
use crate::{Vec2i, Vec3i, Vec2f, Vec3f, Vec4f};
use crate::{veci2f, vecf2i};

use itertools::iproduct;
use crate::shader::IShader;


pub fn line_sweeping_v1(image: &mut TgaImage, v0: Vec2i, v1: Vec2i, v2: Vec2i, color: &TgaColor) {

    fn sort_ascending(mut v0: Vec2i, mut v1: Vec2i, mut v2: Vec2i) -> (Vec2i, Vec2i, Vec2i) {
        if v0.y > v1.y { std::mem::swap(&mut v0, &mut v1); }
        if v1.y > v2.y { std::mem::swap(&mut v1, &mut v2); }
        (v0, v1, v2)
    }

    let (v0, v1, v2) = sort_ascending(v0, v1, v2);
    let (y_start, y_middle, y_end) = (v0.y, v1.y, v2.y);

    for y in y_start..y_end {
        fn calc_x(y: i32, v1: Vec2i, v2: Vec2i) -> i32 {
            (((y - v1.y) * (v2.x - v1.x)) as f32 / (v2.y - v1.y) as f32) as i32 + v1.x
        }

        if y < y_middle {
            let mut x_v0_v1 = calc_x(y, v0, v1);
            let mut x_v0_v2 = calc_x(y, v0, v2);
            if x_v0_v1 > x_v0_v2 { std::mem::swap(&mut x_v0_v1, &mut x_v0_v2); }

            for x in x_v0_v1..x_v0_v2 {
                image.set(x, y, color);
            }
        } else {
            let mut x_v1_v2 = calc_x(y, v1, v2);
            let mut x_v0_v2 = calc_x(y, v0, v2);

            if x_v1_v2 > x_v0_v2 { std::mem::swap(&mut x_v1_v2, &mut x_v0_v2); }
            for x in x_v1_v2..x_v0_v2 {
                image.set(x, y, color);
            }
        }
    }
}

pub fn line_sweeping_v2(image: &mut TgaImage, mut v0: Vec2i, mut v1: Vec2i, mut v2: Vec2i, color: &TgaColor) {
    // sort the vertices, t0, t1, t2 lower−to−upper
    if v0.y > v1.y { std::mem::swap(&mut v0, &mut v1); }
    if v0.y > v2.y { std::mem::swap(&mut v0, &mut v2); }
    if v1.y > v2.y { std::mem::swap(&mut v1, &mut v2); }

    let total_height = (v2.y - v0.y) as f32;

    let segment_height = (v1.y - v0.y + 1) as f32;
    for y in v0.y..v1.y {
        let alpha = (y - v0.y) as f32 / total_height;
        let beta  = (y - v0.y) as f32 / segment_height; // be careful with divisions by zero

        let mut a_x = v0.x + ((v2.x - v0.x) as f32 * alpha) as i32;
        let mut b_x = v0.x + ((v1.x - v0.x) as f32 * beta) as i32;

        if a_x > b_x { std::mem::swap(&mut a_x, &mut b_x); }
        for x in a_x..b_x {
            image.set(x, y, color);
        }
    }

    let segment_height = (v2.y - v1.y) as f32;
    for y in v1.y..v2.y {
        let alpha = (y - v0.y) as f32 / total_height;
        let beta  = (y - v1.y) as f32 / segment_height;

        let mut a_x = v0.x + ((v2.x - v0.x) as f32 * alpha) as i32;
        let mut b_x = v1.x + ((v2.x - v1.x) as f32 * beta)  as i32;

        if a_x > b_x { std::mem::swap(&mut a_x, &mut b_x); }
        for x in a_x..b_x {
            image.set(x, y, color);
        }
    }
}

pub fn line_sweeping_gouraud_shading(image: &mut TgaImage, zbuffer: &mut impl ZBuffer, mut v0: Vec3i, mut v1: Vec3i, mut v2: Vec3i, mut ity0: f32, mut ity1: f32, mut ity2: f32) {
    if v0.y == v1.y && v0.y == v2.y { return }
    if v0.y > v1.y { std::mem::swap(&mut v0, &mut v1); std::mem::swap(&mut ity0, &mut ity1); }
    if v0.y > v2.y { std::mem::swap(&mut v0, &mut v2); std::mem::swap(&mut ity0, &mut ity2); }
    if v1.y > v2.y { std::mem::swap(&mut v1, &mut v2); std::mem::swap(&mut ity1, &mut ity2); }

    let total_height = v2.y - v0.y;

    for i in 0..total_height {
        let second_half = i as i32 > v1.y - v0.y || v1.y == v0.y;
        let segment_height = if second_half { v2.y - v1.y } else { v1.y - v0.y } as f32;

        let alpha = i as f32 / total_height as f32;
        let beta = if second_half { i - (v1.y - v0.y) } else { i } as f32 / segment_height;

        let mut a = vecf2i(veci2f(v0) + (veci2f(v2 - v0) * alpha));
        let mut b = if second_half {
            vecf2i(veci2f(v1) + (veci2f(v2 - v1) * beta))
        } else {
            vecf2i(veci2f(v0) + (veci2f(v1 - v0) * beta))
        };

        let mut a_intensity = ity0 + (ity2 - ity0) * alpha;
        let mut b_intensity = if second_half { ity1 + (ity2 - ity1) * beta } else { ity0 + (ity1 - ity0) * beta };

        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut a_intensity, &mut b_intensity);
        }

        for x in a.x..b.x {
            let phi = (x - a.x) as f32 / (b.x - a.x) as f32;
            let p = vecf2i(veci2f(a) + veci2f(b - a) * phi);
            let p_intensity = a_intensity + (b_intensity - a_intensity) * phi;

            if p.x >= image.width || p.y >= image.height || p.x < 0 || p.y < 0 { continue }
            if zbuffer.get(p.x as usize, p.y as usize) < p.z as f32 {
                zbuffer.set(x as usize, p.y as usize, p.z as f32);
                image.set(p.x, p.y, &(TgaColor::from_rgb(255, 255, 255) * p_intensity));
            }
        }
    }

}

pub fn barycentric_rasterization_v1(image: &mut TgaImage, pts: [Vec2i; 3], color: &TgaColor) {

    /// https://github.com/ssloy/tinyrenderer/wiki/Lesson-2-Triangle-rasterization-and-back-face-culling
    fn barycentric_local(pts: &[Vec2i], p: Vec2i) -> Vec3f {

        let v1 = Vec3f::new((pts[2].x - pts[0].x) as f32, (pts[1].x - pts[0].x) as f32, (pts[0].x - p.x) as f32);
        let v2 = Vec3f::new((pts[2].y - pts[0].y) as f32, (pts[1].y - pts[0].y) as f32, (pts[0].y - p.y) as f32);

        let u = v1.cross(v2);
        if u.z.abs() < 1.0 {
            /* `pts` and `P` has integer value as coordinates so `abs(u[2])` < 1 means `u[2]` is 0,
            that means triangle is degenerate, in this case return something with negative coordinates */
            Vec3f::new(-1.0, 1.0, 1.0)
        } else {
            Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
        }
    }

    let mut bounding_box_min: Vec2i = Vec2i::new(image.width - 1, image.height - 1);
    let mut bounding_box_max: Vec2i = Vec2i::new(0, 0);

    let clamp: Vec2i = bounding_box_min.clone();
    for i in 0..3 {
        bounding_box_min.x = bounding_box_min.x.min(pts[i].x).max(0);
        bounding_box_max.x = bounding_box_max.x.max(pts[i].x).min(clamp.x);

        bounding_box_min.y = bounding_box_min.y.min(pts[i].y).max(0);
        bounding_box_max.y = bounding_box_max.y.max(pts[i].y).min(clamp.y);
    }

    for (x, y) in iproduct!(bounding_box_min.x..=bounding_box_max.x, bounding_box_min.y..=bounding_box_max.y) {

        let bc_screen = barycentric_local(&pts, Vec2i::new(x, y));
        if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
            continue
        } else {
            image.set(x, y, color);
        }
    }
}

pub trait ZBuffer {
    fn get(&self, i: usize, j: usize) -> f32;
    fn set(&mut self, i: usize, j: usize, v: f32);
}

pub struct ZbufferEx {
    pub buffer: [f32; 800 * 800],
    pub width: usize,
}

impl ZBuffer for ZbufferEx {
    fn get(&self, x: usize, y: usize) -> f32 { self.buffer[x + y * self.width] }
    fn set(&mut self, x: usize, y: usize, v: f32) { self.buffer[x + y * self.width] = v; }
}


fn barycentric(a: Vec3f, b: Vec3f, c: Vec3f, p: Vec2i) -> Vec3f {

    let v1 = Vec3f::new(c.x - a.x, b.x - a.x, a.x - p.x as f32);
    let v2 = Vec3f::new(c.y - a.y, b.y - a.y, a.y - p.y as f32);

    let u = v1.cross(v2);
    if u.z.abs() < 1e-2 {
        // in this case generate negative coordinates, it will be thrown away by the rasterizator
        Vec3f::new(-1.0, 1.0, 1.0)
    } else {
        // don't forget that u[2] is integer. If it is zero then triangle ABC is degenerate
        Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

pub fn barycentric_rasterization_v2(image: &mut TgaImage, zbuffer: &mut impl ZBuffer, pts: [Vec3f; 3], color: &TgaColor) {

    // https://github.com/ssloy/tinyrenderer/wiki/Lesson-3-Hidden-faces-removal-(z-buffer)

    use std::f32::{MAX, MIN};
    let mut bounding_box_min: Vec2f = Vec2f::new(MAX, MAX);
    let mut bounding_box_max: Vec2f = Vec2f::new(MIN, MIN);

    let clamp: Vec2f = Vec2f::new((image.width - 1) as f32, (image.height - 1) as f32);
    for i in 0..3 {
        bounding_box_min.x = f32::max(f32::min(bounding_box_min.x, pts[i].x), 0.0);
        bounding_box_max.x = f32::min(f32::max(bounding_box_max.x, pts[i].x), clamp.x);

        bounding_box_min.y = f32::max(f32::min(bounding_box_min.y, pts[i].y), 0.0);
        bounding_box_max.y = f32::min(f32::max(bounding_box_max.y, pts[i].y), clamp.y);
    }

    let bounding_box_min = Vec2i::new(bounding_box_min.x as i32, bounding_box_min.y as i32);
    let bounding_box_max = Vec2i::new(bounding_box_max.x as i32, bounding_box_max.y as i32);

    for (x, y) in iproduct!(bounding_box_min.x..=bounding_box_max.x, bounding_box_min.y..=bounding_box_max.y) {
        let bc_screen = barycentric(pts[0], pts[1], pts[2], Vec2i::new(x, y));
        if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
            continue
        } else {
            let z: f32 = pts[0].z * bc_screen.x + pts[1].z * bc_screen.y + pts[2].z * bc_screen.z;
            if zbuffer.get(x as usize, y as usize) < z {
                zbuffer.set(x as usize, y as usize, z);
                image.set(x, y, color);
            }
        }
    }
}

pub fn barycentric_rasterization_diffuse(image: &mut TgaImage, zbuffer: &mut impl ZBuffer, pts: [Vec3f; 3], uvs: [Vec2f; 3], diffuse: &TgaImage, intensity: f32) -> std::io::Result<()> {

    // https://github.com/ssloy/tinyrenderer/wiki/Lesson-3-Hidden-faces-removal-(z-buffer)

    use std::f32::{MAX, MIN};
    let mut bounding_box_min: Vec2f = Vec2f::new(MAX, MAX);
    let mut bounding_box_max: Vec2f = Vec2f::new(MIN, MIN);

    let clamp: Vec2f = Vec2f::new((image.width - 1) as f32, (image.height - 1) as f32);
    for i in 0..3 {
        bounding_box_min.x = f32::max(f32::min(bounding_box_min.x, pts[i].x), 0.0);
        bounding_box_max.x = f32::min(f32::max(bounding_box_max.x, pts[i].x), clamp.x);

        bounding_box_min.y = f32::max(f32::min(bounding_box_min.y, pts[i].y), 0.0);
        bounding_box_max.y = f32::min(f32::max(bounding_box_max.y, pts[i].y), clamp.y);
    }

    let bounding_box_min = Vec2i::new(bounding_box_min.x as i32, bounding_box_min.y as i32);
    let bounding_box_max = Vec2i::new(bounding_box_max.x as i32, bounding_box_max.y as i32);

    for (x, y) in iproduct!(bounding_box_min.x..=bounding_box_max.x, bounding_box_min.y..=bounding_box_max.y) {
        let bc_screen = barycentric(pts[0], pts[1], pts[2], Vec2i::new(x, y));
        if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
            continue
        } else {
            let z: f32 = pts[0].z * bc_screen.x + pts[1].z * bc_screen.y + pts[2].z * bc_screen.z;
            if zbuffer.get(x as usize, y as usize) < z {
                zbuffer.set(x as usize, y as usize, z);

                // https://github.com/ssloy/tinyrenderer/wiki/Lesson-2-Triangle-rasterization-and-back-face-culling#the-method-i-adopt-for-my-code
                // P = (1 - u - v)A + uB + vC
                let uv: Vec2f = bc_screen.x * uvs[0] + bc_screen.y * uvs[1] + bc_screen.z * uvs[2];
                let color = diffuse.get(uv.x as i32, uv.y as i32)? * intensity;
                image.set(x, y, &color);
            }
        }
    }

    Ok(())
}

// barycentric rasterization
pub fn triangle(image: &mut TgaImage, shader: &impl IShader, zbuffer: &mut impl ZBuffer, pts: [Vec4f; 3]) {

    use std::f32::{MAX, MIN};
    let mut bounding_box_min: Vec2f = Vec2f::new(MAX, MAX);
    let mut bounding_box_max: Vec2f = Vec2f::new(MIN, MIN);

    let clamp: Vec2f = Vec2f::new((image.width - 1) as f32, (image.height - 1) as f32);
    for i in 0..3 {
        bounding_box_min.x = f32::max(f32::min(bounding_box_min.x, pts[i].x / pts[i].w), 0.0);
        bounding_box_max.x = f32::min(f32::max(bounding_box_max.x, pts[i].x / pts[i].w), clamp.x);

        bounding_box_min.y = f32::max(f32::min(bounding_box_min.y, pts[i].y / pts[i].w), 0.0);
        bounding_box_max.y = f32::min(f32::max(bounding_box_max.y, pts[i].y / pts[i].w), clamp.y);
    }

    let bounding_box_min = Vec2i::new(bounding_box_min.x as i32, bounding_box_min.y as i32);
    let bounding_box_max = Vec2i::new(bounding_box_max.x as i32, bounding_box_max.y as i32);

    for (x, y) in iproduct!(bounding_box_min.x..=bounding_box_max.x, bounding_box_min.y..=bounding_box_max.y) {
        let bc: Vec3f = barycentric(
            pts[0].homogenized().xyz(),
            pts[1].homogenized().xyz(),
            pts[2].homogenized().xyz(),
            Vec2i::new(x, y)
        );
        let z: f32 = pts[0].z * bc.x + pts[1].z * bc.y + pts[2].z * bc.z;
        let w: f32 = pts[0].w * bc.x + pts[1].w * bc.y + pts[2].w * bc.z;
        let fragment_depth = (z / w).max(0.0).min(255.0);

        if bc.x < 0.0 || bc.y < 0.0 || bc.z < 0.0 || zbuffer.get(x as usize, y as usize) > fragment_depth {
            continue
        } else if let Some(color) = shader.fragment(bc) {
            zbuffer.set(x as usize, y as usize, fragment_depth);
            image.set(x, y, &color);
        }
    }
}
