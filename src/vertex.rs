use crate::math::Vec3;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec3) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
        }
    }
}
