use crate::vertex::Vertex;
use crate::shaders::Fragment;
use crate::framebuffer::{Framebuffer, Color};
use crate::math::Vec3;

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, framebuffer: &mut Framebuffer, fragment_shader: fn(&Fragment) -> Color) {
    let (a, b, c) = (v1, v2, v3);
    
    let min_x = a.position.x.min(b.position.x).min(c.position.x).max(0.0) as usize;
    let min_y = a.position.y.min(b.position.y).min(c.position.y).max(0.0) as usize;
    let max_x = a.position.x.max(b.position.x).max(c.position.x).min((framebuffer.width - 1) as f32) as usize;
    let max_y = a.position.y.max(b.position.y).max(c.position.y).min((framebuffer.height - 1) as f32) as usize;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            
            let (w1, w2, w3) = barycentric_coordinates(&p, &a.position, &b.position, &c.position);
            
            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let depth = w1 * a.position.z + w2 * b.position.z + w3 * c.position.z;
                
                if !depth.is_finite() || depth < 0.0 {
                    continue;
                }
                
                let normal = (w1 * a.normal + w2 * b.normal + w3 * c.normal).normalize();
                let tex_coords = w1 * a.tex_coords + w2 * b.tex_coords + w3 * c.tex_coords;
                
                let fragment = Fragment::new(
                    Vec3::new(x as f32, y as f32, depth),
                    normal,
                    depth,
                    tex_coords,
                );
                
                let color = fragment_shader(&fragment);
                framebuffer.point_with_color(x, y, depth, color);
            }
        }
    }
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;
    
    if denom.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}
