
use crate::tga::{TgaImage, TgaColor};

#[inline]
fn abs(v1: usize, v2: usize) -> usize {
    if v1 < v2 { v2 - v1 } else { v1 - v2 }
}


pub fn line_segment_v1(image: &mut TgaImage, x0: usize, y0: usize, x1: usize, y1: usize, color: TgaColor) -> std::io::Result<()> {

    for i in 0..100 {
        let t = i as f32 * 0.01;
        let x = x0 + ((x1 - x0) as f32 * t) as usize;
        let y = y0 + ((y1 - y0) as f32 * t) as usize;

        image.set(x, y, &color);
    }
    Ok(())
}

pub fn line_segment_v2(image: &mut TgaImage, mut x0: usize, mut y0: usize, mut x1: usize, mut y1: usize, color: TgaColor) -> std::io::Result<()> {

    let mut steep = false;

    if abs(x0, x1) < abs(y0, y1) {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    if steep {
        for x in x0..x1 {
            let t = (x - x0) as f32 / (x1 - x0) as f32;
            let y = (y0 as f32 * (1.0 - t) + y1 as f32 * t) as usize;
            image.set(y, x, &color); // if transposed, de-transpose
        }
    } else {
        for x in x0..x1 {
            let t = (x - x0) as f32 / (x1 - x0) as f32;
            let y = (y0 as f32 * (1.0 - t) + y1 as f32 * t) as usize;
            image.set(x, y, &color);
        }
    }

    Ok(())
}

pub fn line_segment_v3(image: &mut TgaImage, mut x0: usize, mut y0: usize, mut x1: usize, mut y1: usize, color: TgaColor) -> std::io::Result<()> {

    let mut steep = false;

    if abs(x0, x1) < abs(y0, y1) {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = abs(y1, y0);

    let d_error = dy as f32 / dx as f32;
    let mut error = 0.0;
    let mut y = y0;

    if steep {
        for x in x0..x1 {
            image.set(y, x, &color); // if transposed, de-transpose

            error += d_error;
            if error > 0.5 {
                y = if y1 > y0 { y + 1 } else { y - 1 };
                error -= 1.0;
            }
        }
    } else {
        for x in x0..x1 {
            image.set(x, y, &color);

            error += d_error;
            if error > 0.5 {
                y = if y1 > y0 { y + 1 } else { y - 1 };
                error -= 1.0;
            }
        }
    }

    Ok(())
}
