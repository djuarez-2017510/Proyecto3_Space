use crate::math::Vec3;
use crate::vertex::Vertex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut positions: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec3> = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<usize> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse().unwrap_or(0.0);
                        let y: f32 = parts[2].parse().unwrap_or(0.0);
                        let z: f32 = parts[3].parse().unwrap_or(0.0);
                        positions.push(Vec3::new(x, y, z));
                    }
                }
                "vn" => {
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse().unwrap_or(0.0);
                        let y: f32 = parts[2].parse().unwrap_or(0.0);
                        let z: f32 = parts[3].parse().unwrap_or(0.0);
                        normals.push(Vec3::new(x, y, z));
                    }
                }
                "vt" => {
                    if parts.len() >= 3 {
                        let u: f32 = parts[1].parse().unwrap_or(0.0);
                        let v: f32 = parts[2].parse().unwrap_or(0.0);
                        tex_coords.push(Vec3::new(u, v, 0.0));
                    }
                }
                "f" => {
                    if parts.len() >= 4 {
                        for i in 1..parts.len() {
                            let face_parts: Vec<&str> = parts[i].split('/').collect();
                            
                            let pos_idx: usize = face_parts[0].parse::<usize>().unwrap_or(1) - 1;
                            let tex_idx: usize = if face_parts.len() > 1 && !face_parts[1].is_empty() {
                                face_parts[1].parse::<usize>().unwrap_or(1) - 1
                            } else {
                                0
                            };
                            let norm_idx: usize = if face_parts.len() > 2 {
                                face_parts[2].parse::<usize>().unwrap_or(1) - 1
                            } else {
                                0
                            };

                            let position = if pos_idx < positions.len() {
                                positions[pos_idx]
                            } else {
                                Vec3::new(0.0, 0.0, 0.0)
                            };

                            let normal = if norm_idx < normals.len() {
                                normals[norm_idx]
                            } else {
                                Vec3::new(0.0, 1.0, 0.0)
                            };

                            let tex_coord = if tex_idx < tex_coords.len() {
                                tex_coords[tex_idx]
                            } else {
                                Vec3::new(0.0, 0.0, 0.0)
                            };

                            vertices.push(Vertex::new(position, normal, tex_coord));
                            indices.push(vertices.len() - 1);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Obj { vertices, indices })
    }
}
