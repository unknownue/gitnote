
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::{Vec2f, Vec3f};
use crate::tga::TgaImage;


#[derive(Debug, Clone, Default)]
pub struct Vertex {
    pub position: Vec3f,
    pub normal  : Vec3f,
    pub uv      : Vec2f,
}

#[derive(Debug)]
pub struct ObjMesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<[usize; 3]>,

    pub diffuse_map : TgaImage,
//    normal_map  : TgaImage,
//    specular_map: TgaImage,
}

impl ObjMesh {

    pub fn load_mesh(path: impl AsRef<Path>) -> std::io::Result<ObjMesh> {

        let obj_file = File::open(path.as_ref())?;
        let reader = BufReader::new(obj_file);

        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for line in reader.lines() {

            let line = line?;
            let mut line_splits = line.split_ascii_whitespace();

            if let Some(property) = line_splits.next() {
                match property {
                    | "v" => {
                        let mut position = Vec3f::zero();
                        position.x = line_splits.next().and_then(|x| x.parse().ok()).unwrap();
                        position.y = line_splits.next().and_then(|y| y.parse().ok()).unwrap();
                        position.z = line_splits.next().and_then(|z| z.parse().ok()).unwrap();
                        vertices.push(Vertex { position, ..Default::default() });
                    },
                    | "vt" => {
                        let mut tex = Vec2f::zero();
                        tex[0] = line_splits.next().and_then(|u| u.parse().ok()).unwrap();
                        tex[1] = line_splits.next().and_then(|v| v.parse().ok()).unwrap();
                        uvs.push(tex);
                    },
                    | "vn" => {
                        let mut nor = Vec3f::zero();
                        nor[0] = line_splits.next().and_then(|x| x.parse().ok()).unwrap();
                        nor[1] = line_splits.next().and_then(|y| y.parse().ok()).unwrap();
                        nor[2] = line_splits.next().and_then(|z| z.parse().ok()).unwrap();
                        normals.push(nor);
                    },
                    | "f" => {
                        let mut face = [0_usize; 3];
                        let mut i = 0;

                        while let Some(indices_split) = line_splits.next() {
                            let mut indices_split = indices_split.split('/');

                            // in wavefront obj all indices start at 1, not zero
                            let pos_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;
                            let tex_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;
                            let nor_index = indices_split.next().and_then(|i| i.parse::<usize>().ok()).unwrap() - 1;

                            vertices[pos_index].normal = normals[nor_index];
                            vertices[pos_index].uv = uvs[tex_index];
                            face[i] = pos_index;
                            i += 1;
                        }

                        faces.push(face);
                    },
                    | _ => {}
                }
            }
        }

        ObjMesh::print_help_message(path, &vertices, &faces);

        let mesh = ObjMesh {
            vertices, faces,
            diffuse_map: TgaImage::unset(),
        };
        Ok(mesh)
    }

    pub fn load_diffuse_map(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        self.diffuse_map = TgaImage::from_path(path)?;
        self.diffuse_map.flip_vertically();
        Ok(())
    }

    fn print_help_message(path: impl AsRef<Path>, vertices: &[Vertex], faces: &[[usize; 3]]) {
        println!("-------------------------------------------------------------");
        println!("Load mesh from: {}", path.as_ref().to_str().expect("Invalid Path"));
        println!("\t vertices  count: {}", vertices.len());
        println!("\t triangles count: {}", faces.len());
        println!("-------------------------------------------------------------");
    }
}