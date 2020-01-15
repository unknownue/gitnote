
use std::ops::{Index, IndexMut, Mul};
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use itertools::izip;


// #[repr(packed)] is equivalent to #[repr(packed(1))]
#[repr(packed)]
struct TgaHeader {
    id_length   : u8,
    colormaptype: u8,
    datatypecode: u8,
    colormap_origin: i16,
    colormap_length: i16,
    colormap_depth : u8,
    x_origin: i16,
    y_origin: i16,
    width : i16,
    height: i16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl TgaHeader {

    fn read_header(reader: &mut impl Read) -> std::io::Result<TgaHeader> {

        let id_length        = reader.read_u8()?;
        let colormaptype     = reader.read_u8()?;
        let datatypecode     = reader.read_u8()?;
        let colormap_origin  = reader.read_i16::<LittleEndian>()?;
        let colormap_length  = reader.read_i16::<LittleEndian>()?;
        let colormap_depth   = reader.read_u8()?;
        let x_origin         = reader.read_i16::<LittleEndian>()?;
        let y_origin         = reader.read_i16::<LittleEndian>()?;
        let width            = reader.read_i16::<LittleEndian>()?;
        let height           = reader.read_i16::<LittleEndian>()?;
        let bits_per_pixel   = reader.read_u8()?;
        let image_descriptor = reader.read_u8()?;

        let header = TgaHeader {
            id_length, colormaptype, datatypecode, colormap_origin, colormap_length, colormap_depth,
            x_origin, y_origin, width, height, bits_per_pixel, image_descriptor,
        };
        Ok(header)
    }

    fn write_header(&self, writer: &mut impl Write) -> std::io::Result<()> {

        writer.write_u8(self.id_length)?;
        writer.write_u8(self.colormaptype)?;
        writer.write_u8(self.datatypecode)?;
        writer.write_i16::<LittleEndian>(self.colormap_origin)?;
        writer.write_i16::<LittleEndian>(self.colormap_length)?;
        writer.write_u8(self.colormap_depth)?;
        writer.write_i16::<LittleEndian>(self.x_origin)?;
        writer.write_i16::<LittleEndian>(self.y_origin)?;
        writer.write_i16::<LittleEndian>(self.width)?;
        writer.write_i16::<LittleEndian>(self.height)?;
        writer.write_u8(self.bits_per_pixel)?;
        writer.write_u8(self.image_descriptor)?;

        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct TgaColor {
    bgra: [u8; 4],
    format: TgaFormat,
}

impl TgaColor {

    pub fn zeros() -> TgaColor {
        TgaColor {
            bgra: [0; 4],
            format: TgaFormat::RGBA,
        }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> TgaColor {
        TgaColor {
            bgra: [r, g, b, 0],
            format: TgaFormat::RGB,
        }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> TgaColor {
        TgaColor {
            bgra: [r, g, b, a],
            format: TgaFormat::RGBA,
        }
    }

    #[allow(unused)]
    pub fn from_greyscale(v: u8) -> TgaColor {
        TgaColor {
            bgra: [v, 0, 0, 0],
            format: TgaFormat::Grayscale,
        }
    }
}

impl Index<usize> for TgaColor {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.bgra[i]
    }
}

impl IndexMut<usize> for TgaColor {

    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.bgra[index]
    }
}

impl Mul<f32> for TgaColor {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let rhs = rhs.max(0.0).min(1.0);

        TgaColor {
            bgra: [
                (self.bgra[0] as f32 * rhs) as u8,
                (self.bgra[1] as f32 * rhs) as u8,
                (self.bgra[2] as f32 * rhs) as u8,
                (self.bgra[3] as f32 * rhs) as u8,
            ],
            format: self.format,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum TgaFormat {
    Grayscale = 1,
    RGB       = 3,
    RGBA      = 4,
}


#[derive(Debug, Clone)]
pub struct TgaImage {
    data: Vec<u8>,
    pub width : i32,
    pub height: i32,
    bytes_per_pixel: usize,
}

impl TgaImage {

    pub fn unset() -> TgaImage {
        TgaImage { data: vec![], width: 0, height: 0, bytes_per_pixel: 0 }
    }

    pub fn new(width: i32, height: i32, format: TgaFormat) -> TgaImage {
        
        let bytes_per_pixel = format as usize;
        let nbytes = (width * height) as usize * bytes_per_pixel;

        TgaImage {
            data: vec![0; nbytes],
            width, height, bytes_per_pixel,
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<TgaImage> {

        let mut file = File::open(path)?;
        let header = TgaHeader::read_header(&mut file)
            .expect("An error occurred while reading header!");

        let width  = header.width  as i32;
        let height = header.height as i32;
        let bytes_per_pixel = header.bits_per_pixel >> 3;

        if width == 0 || height == 0 ||
            (bytes_per_pixel != TgaFormat::Grayscale as u8 && bytes_per_pixel != TgaFormat::RGB as u8 && bytes_per_pixel != TgaFormat::RGBA as u8) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "An error occured while reading the data!"))
        }

        let bytes_per_pixel = bytes_per_pixel as usize;
        let data = if header.datatypecode == 3 || header.datatypecode == 2 {
            let mut data = vec![0; (width * height) as usize * bytes_per_pixel];
            let _bytes_read = file.read_exact(&mut data)?;
            data
        } else if header.datatypecode == 10 || header.datatypecode == 11 {
            TgaImage::load_rle_data(&mut file, width, height, bytes_per_pixel)?
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unknown file format({})", header.datatypecode)))
        };

        println!("Read TGA file: {}x{}/{}", width, height, bytes_per_pixel);

        let mut tga_image = TgaImage { data, width, height, bytes_per_pixel };

        if header.image_descriptor & 0x20 == 0 {
            tga_image.flip_vertically();
        }
        if header.image_descriptor & 0x10 != 0 {
            tga_image.flip_horizontally()?;
        }

        Ok(tga_image)
    }

    pub fn flip_vertically(&mut self) {

        let bytes_per_line = self.width as usize * self.bytes_per_pixel;

        let height = self.height as usize;
        let half = height >> 1;

        for j in 0..half {
            let l1 = j * bytes_per_line;
            let l2 = (height - 1 - j) * bytes_per_line;

            unsafe {
                std::ptr::swap_nonoverlapping(&mut self.data[l1], &mut self.data[l2], bytes_per_line);
            }
        }
    }

    pub fn flip_horizontally(&mut self) -> std::io::Result<()> {

        let half = self.width >> 1;
        for (i, j) in izip!(0..half, 0..self.height) {
            let c1 = self.get(i, j)?;
            let c2 = self.get(self.width - 1 - i, j)?;
            self.set(i, j, &c2);
            self.set(self.width - 1 - i, j, &c1);

        }

        Ok(())
    }

    pub fn write_tga_file(&self, path: impl AsRef<Path>, rle: bool) -> std::io::Result<()> {

        let mut file = File::create(path)?;

        let header = TgaHeader {
            id_length   : 0,
            colormaptype: 0,
            datatypecode: if self.bytes_per_pixel == TgaFormat::Grayscale as _ {
                if rle { 11 } else { 3 }
            } else {
                if rle { 10 } else { 2 }
            },
            colormap_origin: 0,
            colormap_length: 0,
            colormap_depth : 0,
            x_origin: 0,
            y_origin: 0,
            width : self.width  as i16,
            height: self.height as i16,
            bits_per_pixel: (self.bytes_per_pixel << 3) as u8,
            image_descriptor: 0x20,
        };
        
        header.write_header(&mut file)
            .expect("Can't dump the tga file!");

        if rle {
            self.unload_rle_data(&mut file)?
        } else {
//            for i in 0..100 {
//                for j in 0..100 {
//                    let r = self.data[(i * 100 + j) * 3 + 0];
//                    let g = self.data[(i * 100 + j) * 3 + 1];
//                    let b = self.data[(i * 100 + j) * 3 + 2];
//                    print!("({}, {}, {}) ", r, g, b);
//                }
//                println!();
//            }
            file.write(&self.data)
                .expect("Can't unload raw data!");
        }

        let developer_area_ref: [u8; 4] = [0, 0, 0, 0];
        file.write(&developer_area_ref)?;
        let extension_area_ref: [u8; 4] = [0, 0, 0, 0];
        file.write(&extension_area_ref)?;
        let footer: [u8; 18] = [b'T', b'R', b'U', b'E', b'V', b'I', b'S', b'I', b'O', b'N', b'-', b'X', b'F', b'I', b'L', b'E', b'.', b'\0'];
        file.write(&footer)?;
        
        Ok(())
    }

    fn load_rle_data(file: &mut impl Read, width: i32, height: i32, bytes_per_pixel: usize) -> std::io::Result<Vec<u8>> {

        let pixel_count = (width * height) as usize;
        let mut data = vec![0; pixel_count * bytes_per_pixel];
        let mut current_pixel = 0;
        let mut current_byte = 0;
        let mut color_buffer = TgaColor::zeros();

        loop {
            let mut chunk_header = file.read_u8()?;
            
            if chunk_header < 128 {
                chunk_header += 1;
                for _ in 0..chunk_header {
                    for i in 0..bytes_per_pixel {
                        color_buffer[i] = file.read_u8()?;
                    }
                    for i in 0..bytes_per_pixel {
                        data[current_byte] = color_buffer[i];
                        current_byte += 1;
                    }
                    current_pixel += 1;
                    if current_pixel > pixel_count {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Too many pixels read!"))
                    }
                }
            } else {
                chunk_header -= 127;
                for i in 0..bytes_per_pixel {
                    color_buffer[i] = file.read_u8()?;
                }

                for _ in 0..chunk_header {
                    for i in 0..bytes_per_pixel {
                        data[current_byte] = color_buffer[i];
                        current_byte += 1;
                    }
                    current_pixel += 1;
                    if current_pixel > pixel_count {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Too many pixels read!"))
                    }
                }
            }

            if current_pixel >= pixel_count {
                break
            }
        }

        Ok(data)
    }

    fn unload_rle_data(&self, file: &mut impl Write) -> std::io::Result<()> {

        const MAX_CHUNK_LENGTH: usize = 128;
        let pixel_count = (self.width * self.height) as usize;
        let mut current_pixel = 0;

        while current_pixel < pixel_count {
            let chunk_start = current_pixel * self.bytes_per_pixel;
            let mut current_byte = current_pixel * self.bytes_per_pixel;
            let mut run_length = 1;
            let mut raw = true;

            while current_pixel + run_length < pixel_count && run_length < MAX_CHUNK_LENGTH {
                let mut success_eq = true;
                for i in 0..self.bytes_per_pixel {
                    if success_eq == false { break }
                    success_eq = self.data[current_byte + i] == self.data[current_byte + i + self.bytes_per_pixel];
                }

                current_byte += self.bytes_per_pixel;
                if run_length == 1 {
                    raw = !success_eq;
                }
                if raw && success_eq {
                    run_length -= 1;
                    break
                }
                if !raw && !success_eq {
                    break
                }
                run_length += 1;
            }

            current_pixel += run_length;

            file.write_u8((if raw { run_length - 1 } else { run_length + 127 }) as u8)?;
            file.write(&self.data[chunk_start..(chunk_start + if raw { run_length * self.bytes_per_pixel} else {self.bytes_per_pixel})])?;
        }

        Ok(())
    }

    pub fn set(&mut self, x: i32, y: i32, color: &TgaColor) {
        if x < self.width && y < self.height {
            let location = (x + y * self.width) as usize * self.bytes_per_pixel;
            for i in 0..self.bytes_per_pixel {
                self.data[location + i] = color[i];
            }
        }
    }

    pub fn get(&self, x: i32, y: i32) -> std::io::Result<TgaColor> {
        if x >= self.width || y >= self.height {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Color location is out of bound!"))
        } else {
            let mut color = TgaColor::from_rgba(0, 0, 0, 0);
            let location = (x + y * self.width) as usize * self.bytes_per_pixel;
            for i in 0..self.bytes_per_pixel {
                color[i] = self.data[location + i];
            }
            Ok(color)
        }
    }
}
