
use criterion::{criterion_group, criterion_main, Criterion};
use tinyrenderer::tga::{TgaColor, TgaImage, TgaFormat};
use tinyrenderer::mesh::ObjMesh;
use tinyrenderer::rasterization::ZBuffer;
use tinyrenderer::Vec3f;


const GREEN: TgaColor = TgaColor::from_rgb(0, 255, 0);
const RED  : TgaColor = TgaColor::from_rgb(0, 0, 255);
const WHITE: TgaColor = TgaColor::from_rgb(255, 255, 255);


pub fn linesweeping_benchmark(c: &mut Criterion) {

    use tinyrenderer::rasterization::{line_sweeping_v1, line_sweeping_v2};

    let mut group = c.benchmark_group("Line sweeping Algorithm");
    let mut image = TgaImage::new(200, 200, TgaFormat::RGB);

    let t0 = [Vec2i::new(10, 70), Vec2i::new(50, 160), Vec2i::new(70, 80)];
    let t1 = [Vec2i::new(180, 50), Vec2i::new(150, 1), Vec2i::new(70, 180)];
    let t2 = [Vec2i::new(180, 150), Vec2i::new(120, 160), Vec2i::new(130, 180)];

    group.bench_function("v1", |b| b.iter(|| {
        line_sweeping_v1(&mut image, t0[0], t0[1], t0[2], &RED);
        line_sweeping_v1(&mut image, t1[0], t1[1], t1[2], &WHITE);
        line_sweeping_v1(&mut image, t2[0], t2[1], t2[2], &GREEN);
    }));
    group.bench_function("v2", |b| b.iter(|| {
        line_sweeping_v2(&mut image, t0[0], t0[1], t0[2], &RED);
        line_sweeping_v2(&mut image, t1[0], t1[1], t1[2], &WHITE);
        line_sweeping_v2(&mut image, t2[0], t2[1], t2[2], &GREEN);
    }));
}

fn barycentric_benchmark() {

    use tinyrenderer::rasterization::{barycentric_rasterization_v1, barycentric_rasterization_v2};

    let mut group = c.benchmark_group("Barycentric Rasterization Algorithm");
    let mut image = TgaImage::new(800, 800, TgaFormat::RGB);
    let mut mesh = ObjMesh::load_mesh("./assets/african_head/african_head.obj")?;
    mesh.load_diffuse_map("./assets/african_head/african_diffuse.tga")?;

    group.bench_function("v1", |b| b.iter(|| {
        let light_dir = Vec3f::new(0.0, 0.0, -1.0);
        for face in mesh.faces.iter() {
            let (screen_coords, world_coords) = {
                let world_coords = [
                    mesh.vertices[face[0]].position,
                    mesh.vertices[face[1]].position,
                    mesh.vertices[face[2]].position,
                ];
                let screen_coords = [
                    Vec2i::new(((world_coords[0].x + 1.0) * 400.0) as i32, ((world_coords[0].y + 1.0) * 400.0) as i32),
                    Vec2i::new(((world_coords[1].x + 1.0) * 400.0) as i32, ((world_coords[1].y + 1.0) * 400.0) as i32),
                    Vec2i::new(((world_coords[2].x + 1.0) * 400.0) as i32, ((world_coords[2].y + 1.0) * 400.0) as i32),
                ];
                (screen_coords, world_coords)
            };

            let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]).normalized();
            let intensity = n.dot(light_dir);
            if intensity > 0.0 {
                let intensity = (intensity * 255.0) as u8;
                let color = TgaColor::from_rgb(intensity, intensity, intensity);
                barycentric_rasterization_v1(&mut image, screen_coords, &color);
            }
        }
    }));


    group.bench_function("v2", |b| b.iter(|| {

        struct ZbufferEx { buffer: [f32; 800 * 800], width: usize }
        impl ZBuffer for ZbufferEx {
            fn get(&self, x: usize, y: usize) -> f32 { self.buffer[x + y * self.width] }
            fn set(&mut self, x: usize, y: usize, v: f32) { self.buffer[x + y * self.width] = v; }
        }

        let mut z_buffer = ZbufferEx { buffer: [std::f32::MIN; 800 * 800], width: 800 };

        for face in mesh.faces.iter() {
            fn world_to_screen(world: &Vec3f) -> Vec3f {
                Vec3f::new(((world.x + 1.0) * 400.0 + 0.5).floor(), ((world.y + 1.0) * 400.0 + 0.5).floor(), world.z)
            }

            let world_coords = [
                mesh.vertices[face[0]].position,
                mesh.vertices[face[1]].position,
                mesh.vertices[face[2]].position,
            ];

            let screen_coords = [
                world_to_screen(&world_coords[0]),
                world_to_screen(&world_coords[1]),
                world_to_screen(&world_coords[2]),
            ];

            let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]).normalized();
            let intensity = n.dot(light_dir);
            if intensity > 0.0 {
                let intensity = (intensity * 255.0) as u8;
                let color = TgaColor::from_rgb(intensity, intensity, intensity);
                barycentric_rasterization_v2(&mut image, &mut z_buffer, screen_coords, &color);
            }
        }
    }));

}

criterion_group!(benches, linesweeping_benchmark, barycentric_benchmark);
criterion_main!(benches);
