use crate::math::{Vec3, Vec4, Mat4};
use crate::vertex::Vertex;
use crate::framebuffer::Color;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
}

impl Uniforms {
    pub fn new() -> Self {
        Uniforms {
            model_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            projection_matrix: Mat4::identity(),
            viewport_matrix: Mat4::identity(),
            time: 0.0,
        }
    }
}

pub struct Fragment {
    pub position: Vec3,
    pub normal: Vec3,
    pub depth: f32,
    pub color: Color,
    pub tex_coords: Vec3,
}

impl Fragment {
    pub fn new(position: Vec3, normal: Vec3, depth: f32, tex_coords: Vec3) -> Self {
        Fragment {
            position,
            normal,
            depth,
            color: Color::new(255, 255, 255),
            tex_coords,
        }
    }
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix 
        * uniforms.view_matrix 
        * uniforms.model_matrix 
        * position;

    let w = transformed.w;
    
    if w.abs() < 0.0001 {
        return Vertex {
            position: Vec3::new(-10000.0, -10000.0, -10000.0),
            normal: vertex.normal,
            tex_coords: vertex.tex_coords,
        };
    }
    
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * ndc_position;

    let model_mat3 = Mat4::identity(); // Simplificado por ahora
    let normal_matrix = model_mat3;
    
    let transformed_normal = normal_matrix * Vec4::new(
        vertex.normal.x,
        vertex.normal.y,
        vertex.normal.z,
        0.0
    );

    Vertex {
        position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        normal: Vec3::new(transformed_normal.x, transformed_normal.y, transformed_normal.z).normalize(),
        tex_coords: vertex.tex_coords,
    }
}

pub fn create_model_matrix(translation: Vec3, scale_val: f32, rotation: Vec3) -> Mat4 {
    use crate::math::{translate, scale, rotate_x, rotate_y, rotate_z};
    
    let translation_matrix = translate(&translation);
    let scale_matrix = scale(&Vec3::new(scale_val, scale_val, scale_val));
    
    let rotation_x_mat = rotate_x(rotation.x);
    let rotation_y_mat = rotate_y(rotation.y);
    let rotation_z_mat = rotate_z(rotation.z);
    
    let rotation_matrix = rotation_z_mat * rotation_y_mat * rotation_x_mat;
    
    translation_matrix * rotation_matrix * scale_matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    use crate::math::look_at;
    look_at(&eye, &center, &up)
}

pub fn create_perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    use crate::math::perspective;
    perspective(fov, aspect, near, far)
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    use crate::math::viewport;
    viewport(width, height)
}

// Fragment shaders básicos
pub fn basic_fragment_shader(fragment: &Fragment) -> Color {
    // Shader básico con iluminación simple
    let light_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.0);
    
    Color::from_float(intensity, intensity, intensity)
}

pub fn color_fragment_shader(fragment: &Fragment) -> Color {
    // Shader que usa las normales como color
    Color::from_float(
        (fragment.normal.x + 1.0) * 0.5,
        (fragment.normal.y + 1.0) * 0.5,
        (fragment.normal.z + 1.0) * 0.5,
    )
}
