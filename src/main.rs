mod framebuffer;
mod vertex;
mod shaders;
mod triangle;
mod obj;
mod camera;
mod planet_shaders;
mod math;

use framebuffer::{Framebuffer, Color};
use minifb::{Key, Window, WindowOptions};
use math::Vec3;
use std::time::Duration;
use std::f32::consts::PI;

use shaders::{
    Uniforms, 
    vertex_shader,
    Fragment,
    create_model_matrix,
    create_view_matrix,
    create_perspective_matrix,
    create_viewport_matrix,
};

use vertex::Vertex;
use triangle::triangle;
use obj::Obj;
use camera::Camera;
use planet_shaders::*;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const RENDER_WIDTH: usize = 700;
const RENDER_HEIGHT: usize = 525;

struct CelestialBody {
    position: Vec3,
    scale: f32,
    rotation_speed: f32,
    orbit_speed: f32,
    orbit_radius: f32,
    rotation: f32,
    orbit_angle: f32,
    shader: fn(&Fragment) -> Color,
}

impl CelestialBody {
    fn new(orbit_radius: f32, scale: f32, rotation_speed: f32, orbit_speed: f32, shader: fn(&Fragment) -> Color) -> Self {
        CelestialBody {
            position: Vec3::new(orbit_radius, 0.0, 0.0),
            scale,
            rotation_speed,
            orbit_speed,
            orbit_radius,
            rotation: 0.0,
            orbit_angle: 0.0,
            shader,
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.rotation += self.rotation_speed * delta_time;
        self.orbit_angle += self.orbit_speed * delta_time;
        
        self.position.x = self.orbit_radius * self.orbit_angle.cos();
        self.position.z = self.orbit_radius * self.orbit_angle.sin();
    }
}

fn render_skybox(framebuffer: &mut Framebuffer) {
    for y in 0..RENDER_HEIGHT {
        for x in 0..RENDER_WIDTH {
            let seed = (x * 73856093) ^ (y * 19349663);
            let rand_val = ((seed * 1103515245 + 12345) / 65536) % 10000;
            
            if rand_val < 100 {
                let brightness = ((rand_val % 3) as f32 / 2.0) * 0.6 + 0.6;
                let color = Color::from_float(brightness, brightness, brightness);
                framebuffer.point_with_color(x, y, 1000.0, color);
            }
        }
    }
}

fn render_orbit(planet: &CelestialBody, framebuffer: &mut Framebuffer, uniforms: &Uniforms) {
    let segments = 150;
    let orbit_color = Color::new(60, 60, 80);
    
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * 2.0 * PI;
        let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * PI;
        
        let p1 = Vec3::new(
            planet.orbit_radius * angle1.cos(),
            0.0,
            planet.orbit_radius * angle1.sin()
        );
        let p2 = Vec3::new(
            planet.orbit_radius * angle2.cos(),
            0.0,
            planet.orbit_radius * angle2.sin()
        );
        
        use crate::math::Vec4;
        let pos1_4d = Vec4::new(p1.x, p1.y, p1.z, 1.0);
        let pos2_4d = Vec4::new(p2.x, p2.y, p2.z, 1.0);
        
        let transformed1 = uniforms.projection_matrix * uniforms.view_matrix * pos1_4d;
        let transformed2 = uniforms.projection_matrix * uniforms.view_matrix * pos2_4d;
        
        if transformed1.w.abs() < 0.001 || transformed2.w.abs() < 0.001 {
            continue;
        }
        
        let ndc1 = Vec4::new(
            transformed1.x / transformed1.w,
            transformed1.y / transformed1.w,
            transformed1.z / transformed1.w,
            1.0
        );
        let ndc2 = Vec4::new(
            transformed2.x / transformed2.w,
            transformed2.y / transformed2.w,
            transformed2.z / transformed2.w,
            1.0
        );
        
        if ndc1.x.abs() > 2.0 || ndc1.y.abs() > 2.0 || ndc2.x.abs() > 2.0 || ndc2.y.abs() > 2.0 {
            continue;
        }
        
        if ndc1.z < -1.0 || ndc1.z > 1.0 || ndc2.z < -1.0 || ndc2.z > 1.0 {
            continue;
        }
        
        let screen1 = uniforms.viewport_matrix * ndc1;
        let screen2 = uniforms.viewport_matrix * ndc2;
        
        if !screen1.x.is_finite() || !screen1.y.is_finite() || 
           !screen2.x.is_finite() || !screen2.y.is_finite() ||
           !screen1.z.is_finite() || !screen2.z.is_finite() {
            continue;
        }
        let sx1 = screen1.x as i32;
        let sy1 = screen1.y as i32;
        let sx2 = screen2.x as i32;
        let sy2 = screen2.y as i32;
        
        if sx1.abs() > 10000 || sy1.abs() > 10000 || sx2.abs() > 10000 || sy2.abs() > 10000 {
            continue;
        }
        
        draw_line(
            sx1, sy1,
            sx2, sy2,
            framebuffer,
            orbit_color,
            screen1.z
        );
    }
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, framebuffer: &mut Framebuffer, color: Color, depth: f32) {
    if !depth.is_finite() {
        return;
    }
    
    let distance = ((x1 - x0).pow(2) + (y1 - y0).pow(2)) as f32;
    if distance > 1000000.0 {
        return;
    }
    
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;
    
    let mut steps = 0;
    let max_steps = 2000;
    
    loop {
        if x >= 0 && x < RENDER_WIDTH as i32 && y >= 0 && y < RENDER_HEIGHT as i32 {
            framebuffer.point_with_color(x as usize, y as usize, depth, color);
        }
        
        if x == x1 && y == y1 {
            break;
        }
        
        steps += 1;
        if steps > max_steps {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn render_obj_with_shader(
    obj: &Obj, 
    framebuffer: &mut Framebuffer, 
    uniforms: &Uniforms,
    shader: fn(&Fragment) -> Color
) {
    let transformed_vertices: Vec<Vertex> = obj.vertices
        .iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();

    for i in (0..obj.indices.len()).step_by(3) {
        if i + 2 >= obj.indices.len() {
            break;
        }
        
        let idx1 = obj.indices[i];
        let idx2 = obj.indices[i + 1];
        let idx3 = obj.indices[i + 2];
        
        if idx1 >= transformed_vertices.len() || 
           idx2 >= transformed_vertices.len() || 
           idx3 >= transformed_vertices.len() {
            continue;
        }

        let v1 = &transformed_vertices[idx1];
        let v2 = &transformed_vertices[idx2];
        let v3 = &transformed_vertices[idx3];

        if v1.position.z < 0.0 || v2.position.z < 0.0 || v3.position.z < 0.0 {
            continue;
        }

        triangle(v1, v2, v3, framebuffer, shader);
    }
}

fn scale_buffer(src: &[u32], dst: &mut [u32], src_w: usize, src_h: usize, dst_w: usize, dst_h: usize) {
    let x_ratio = (src_w - 1) as f32 / dst_w as f32;
    let y_ratio = (src_h - 1) as f32 / dst_h as f32;
    
    for y in 0..dst_h {
        for x in 0..dst_w {
            let src_x = x as f32 * x_ratio;
            let src_y = y as f32 * y_ratio;
            
            let x0 = src_x.floor() as usize;
            let y0 = src_y.floor() as usize;
            let x1 = (x0 + 1).min(src_w - 1);
            let y1 = (y0 + 1).min(src_h - 1);
            
            let fx = src_x - x0 as f32;
            let fy = src_y - y0 as f32;
            
            let c00 = src[y0 * src_w + x0];
            let c10 = src[y0 * src_w + x1];
            let c01 = src[y1 * src_w + x0];
            let c11 = src[y1 * src_w + x1];
            
            let r0 = ((c00 >> 16) & 0xFF) as f32;
            let g0 = ((c00 >> 8) & 0xFF) as f32;
            let b0 = (c00 & 0xFF) as f32;
            
            let r1 = ((c10 >> 16) & 0xFF) as f32;
            let g1 = ((c10 >> 8) & 0xFF) as f32;
            let b1 = (c10 & 0xFF) as f32;
            
            let r2 = ((c01 >> 16) & 0xFF) as f32;
            let g2 = ((c01 >> 8) & 0xFF) as f32;
            let b2 = (c01 & 0xFF) as f32;
            
            let r3 = ((c11 >> 16) & 0xFF) as f32;
            let g3 = ((c11 >> 8) & 0xFF) as f32;
            let b3 = (c11 & 0xFF) as f32;
            
            let r_top = r0 * (1.0 - fx) + r1 * fx;
            let r_bottom = r2 * (1.0 - fx) + r3 * fx;
            let r = (r_top * (1.0 - fy) + r_bottom * fy) as u32;
            
            let g_top = g0 * (1.0 - fx) + g1 * fx;
            let g_bottom = g2 * (1.0 - fx) + g3 * fx;
            let g = (g_top * (1.0 - fy) + g_bottom * fy) as u32;
            
            let b_top = b0 * (1.0 - fx) + b1 * fx;
            let b_bottom = b2 * (1.0 - fx) + b3 * fx;
            let b = (b_top * (1.0 - fy) + b_bottom * fy) as u32;
            
            dst[y * dst_w + x] = (r << 16) | (g << 8) | b;
        }
    }
}

fn main() {
    // Framebuffer a menor resolución para mejor performance
    let mut framebuffer = Framebuffer::new(RENDER_WIDTH, RENDER_HEIGHT);
    
    // Buffer escalado para mostrar en ventana
    let mut scaled_buffer = vec![0u32; WINDOW_WIDTH * WINDOW_HEIGHT];
    
    let mut window = Window::new(
        "Space System - Rust Graphics",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    framebuffer.set_background_color(Color::new(0, 0, 10));

    // Cargar el modelo de esfera
    let sphere = Obj::load("assets/sphere.obj").expect("No se pudo cargar sphere.obj");
    println!("Sphere loaded: {} vertices, {} indices", sphere.vertices.len(), sphere.indices.len());
    println!("\n=== CONTROLES ===");
    println!("Flechas: Mover cámara (izq/der orbitar, arriba/abajo zoom)");
    println!("Teclas 1-8: Warp a planetas (1=Sol, 2-8=Planetas)");
    println!("Tecla 0: Vista general del sistema");
    println!("ESC: Salir\n");

    // Inicializar cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 10.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    // Crear el Sol
    let mut sun = CelestialBody::new(0.0, 2.0, 0.2, 0.0, sun_shader);

    // Crear planetas (orbit_radius, scale, rotation_speed, orbit_speed, shader)
    let mut planets = vec![
        CelestialBody::new(5.0, 0.4, 1.0, 0.8, rocky_planet_shader),      // Mercurio
        CelestialBody::new(7.0, 0.6, 0.8, 0.6, earth_shader),             // Venus
        CelestialBody::new(10.0, 0.7, 1.2, 0.5, earth_shader),            // Tierra
        CelestialBody::new(13.0, 0.5, 1.1, 0.4, red_planet_shader),       // Marte
        CelestialBody::new(18.0, 1.5, 0.5, 0.2, gas_planet_shader),       // Júpiter
    ];

    let mut time = 0.0f32;
    let mut last_time = std::time::Instant::now();
    
    let mut warping = false;
    let mut warp_progress = 0.0f32;
    let mut warp_start_pos = camera.eye;
    let mut warp_target_pos = camera.eye;
    let warp_duration = 1.5;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;
        
        time += delta_time;

        // Instant warping con teclas numéricas
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            // Warp al sol
            warp_start_pos = camera.eye;
            warp_target_pos = Vec3::new(0.0, 5.0, 8.0);
            camera.center = Vec3::new(0.0, 0.0, 0.0);
            warping = true;
            warp_progress = 0.0;
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) && planets.len() > 0 {
            warp_start_pos = camera.eye;
            warp_target_pos = planets[0].position + Vec3::new(0.0, 2.0, 3.0);
            camera.center = planets[0].position;
            warping = true;
            warp_progress = 0.0;
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) && planets.len() > 1 {
            warp_start_pos = camera.eye;
            warp_target_pos = planets[1].position + Vec3::new(0.0, 2.0, 4.0);
            camera.center = planets[1].position;
            warping = true;
            warp_progress = 0.0;
        }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) && planets.len() > 2 {
            warp_start_pos = camera.eye;
            warp_target_pos = planets[2].position + Vec3::new(0.0, 2.0, 4.0);
            camera.center = planets[2].position;
            warping = true;
            warp_progress = 0.0;
        }
        if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) && planets.len() > 3 {
            warp_start_pos = camera.eye;
            warp_target_pos = planets[3].position + Vec3::new(0.0, 2.0, 3.0);
            camera.center = planets[3].position;
            warping = true;
            warp_progress = 0.0;
        }
        if window.is_key_pressed(Key::Key6, minifb::KeyRepeat::No) && planets.len() > 4 {
            warp_start_pos = camera.eye;
            warp_target_pos = planets[4].position + Vec3::new(0.0, 3.0, 6.0);
            camera.center = planets[4].position;
            warping = true;
            warp_progress = 0.0;
        }
        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
            // Warp a vista general
            warp_start_pos = camera.eye;
            warp_target_pos = Vec3::new(0.0, 15.0, 35.0);
            camera.center = Vec3::new(0.0, 0.0, 0.0);
            warping = true;
            warp_progress = 0.0;
        }

        // Animación de warping
        if warping {
            warp_progress += delta_time / warp_duration;
            if warp_progress >= 1.0 {
                warping = false;
                warp_progress = 1.0;
            }
            
            // Interpolación suave (easing)
            let t = warp_progress;
            let smooth_t = t * t * (3.0 - 2.0 * t); // smoothstep
            
            camera.eye.x = warp_start_pos.x + (warp_target_pos.x - warp_start_pos.x) * smooth_t;
            camera.eye.y = warp_start_pos.y + (warp_target_pos.y - warp_start_pos.y) * smooth_t;
            camera.eye.z = warp_start_pos.z + (warp_target_pos.z - warp_start_pos.z) * smooth_t;
            
            camera.distance = (camera.eye - camera.center).magnitude();
        }

        // Control de cámara (solo cuando no está en warping)
        if !warping {
            if window.is_key_down(Key::Left) {
                camera.orbit_around_center(delta_time * 2.0);
            }
            if window.is_key_down(Key::Right) {
                camera.orbit_around_center(-delta_time * 2.0);
            }
            if window.is_key_down(Key::Up) {
                camera.zoom(-delta_time * 5.0);
            }
            if window.is_key_down(Key::Down) {
                camera.zoom(delta_time * 5.0);
            }
        }

        framebuffer.clear();
        
        // Renderizar skybox (estrellas de fondo)
        render_skybox(&mut framebuffer);

        // Setup uniforms
        let mut uniforms = Uniforms::new();
        
        uniforms.projection_matrix = create_perspective_matrix(
            PI / 3.0,
            RENDER_WIDTH as f32 / RENDER_HEIGHT as f32,
            0.1,
            100.0
        );

        uniforms.view_matrix = create_view_matrix(
            camera.eye,
            camera.center,
            camera.up
        );

        uniforms.viewport_matrix = create_viewport_matrix(RENDER_WIDTH as f32, RENDER_HEIGHT as f32);
        uniforms.time = time;

        // Renderizar órbitas (todas siempre visibles)
        for planet in &planets {
            render_orbit(planet, &mut framebuffer, &uniforms);
        }

        // Renderizar el Sol
        uniforms.model_matrix = create_model_matrix(
            sun.position,
            sun.scale,
            Vec3::new(0.0, sun.rotation, 0.0)
        );
        render_obj_with_shader(&sphere, &mut framebuffer, &uniforms, sun.shader);
        sun.update(delta_time);

        // Renderizar planetas
        for planet in &mut planets {
            planet.update(delta_time);
            
            uniforms.model_matrix = create_model_matrix(
                planet.position,
                planet.scale,
                Vec3::new(0.0, planet.rotation, 0.0)
            );
            
            render_obj_with_shader(&sphere, &mut framebuffer, &uniforms, planet.shader);
        }

        // Escalar el framebuffer de menor a mayor resolución
        scale_buffer(
            &framebuffer.buffer, 
            &mut scaled_buffer, 
            RENDER_WIDTH, 
            RENDER_HEIGHT, 
            WINDOW_WIDTH, 
            WINDOW_HEIGHT
        );

        window
            .update_with_buffer(&scaled_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }
}

