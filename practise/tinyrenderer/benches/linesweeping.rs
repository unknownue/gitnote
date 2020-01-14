
use criterion::{criterion_group, criterion_main, Criterion};
use tinyrenderer::rasterization::{line_sweeping_v1, line_sweeping_v2};
use tinyrenderer::tga::{TgaColor, TgaImage, TgaFormat};


const GREEN: TgaColor = TgaColor::from_rgb(0, 255, 0);
const RED  : TgaColor = TgaColor::from_rgb(0, 0, 255);
const WHITE: TgaColor = TgaColor::from_rgb(255, 255, 255);


pub fn linesweeping_benchmark(c: &mut Criterion) {

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

criterion_group!(benches, linesweeping_benchmark);
criterion_main!(benches);
