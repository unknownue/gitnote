
use criterion::{criterion_group, criterion_main, Criterion};
use tinyrenderer::bresenham::{line_segment_v2, line_segment_v3};
use tinyrenderer::tga::{TgaColor, TgaImage, TgaFormat};


const RED  : TgaColor = TgaColor::from_rgb(0, 0, 255);
const WHITE: TgaColor = TgaColor::from_rgb(255, 255, 255);


pub fn bresenham_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bresenham Algorithm");
    let mut image = TgaImage::new(100, 100, TgaFormat::RGB);

    group.bench_function("v2", |b| b.iter(|| {
        line_segment_v2(&mut image, 13, 20, 80, 40, &WHITE);
    }));
    group.bench_function("v3", |b| b.iter(|| {
        line_segment_v3(&mut image, 13, 20, 80, 40, &RED);
    }));
}

criterion_group!(benches, bresenham_benchmark);
criterion_main!(benches);
