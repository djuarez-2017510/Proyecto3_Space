use crate::math::Vec3;
use crate::shaders::Fragment;
use crate::framebuffer::Color;

// Shader para el Sol - Más brillante y pulsante
pub fn sun_shader(fragment: &Fragment) -> Color {
    let bright = 1.5;  // Aumentado de 1.0
    let flicker = (fragment.position.x * 0.1 + fragment.position.y * 0.1).sin() * 0.15 + 1.0;  // Más pulsación
    
    Color::from_float(
        (bright * flicker).min(1.0),
        (bright * 0.8 * flicker).min(1.0),
        (bright * 0.4 * flicker).min(1.0),
    )
}

pub fn rocky_planet_shader(fragment: &Fragment) -> Color {
    let light_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.3);
    
    let base_color = Vec3::new(0.8, 0.5, 0.4);
    
    let pattern = ((fragment.tex_coords.x * 20.0).sin() * (fragment.tex_coords.y * 20.0).cos()).abs();
    let variation = 0.8 + pattern * 0.2;
    
    Color::from_float(
        base_color.x * intensity * variation * 1.2,
        base_color.y * intensity * variation * 1.2,
        base_color.z * intensity * variation * 1.2
    )
}

pub fn gas_planet_shader(fragment: &Fragment) -> Color {
    let light_dir = Vec3::new(1.0, 0.5, 0.5).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.3);  // Aumentado de 0.1 a 0.3
    
    let bands = (fragment.tex_coords.y * 10.0).sin() * 0.5 + 0.5;
    let turbulence = ((fragment.tex_coords.x * 15.0).sin() * (fragment.tex_coords.y * 3.0).cos()).abs();
    
    let color1 = Vec3::new(1.0, 0.7, 0.5);  // Más claro
    let color2 = Vec3::new(1.0, 0.8, 0.6);  // Más claro
    
    let mixed = color1 * bands + color2 * (1.0 - bands);
    let final_color = mixed * (0.9 + turbulence * 0.1);
    
    Color::from_float(
        final_color.x * intensity * 1.2,
        final_color.y * intensity * 1.2,
        final_color.z * intensity * 1.2
    )
}

pub fn earth_shader(fragment: &Fragment) -> Color {
    let light_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.3);
    
    let ocean = ((fragment.tex_coords.x * 8.0).sin() * (fragment.tex_coords.y * 8.0).cos()).abs();
    
    let blue = Vec3::new(0.3, 0.5, 1.0);   // Más brillante
    let green = Vec3::new(0.4, 0.8, 0.4);  // Más brillante
    
    let color = if ocean > 0.5 { blue } else { green };
    
    Color::from_float(
        color.x * intensity * 1.3,  // +30% brillo
        color.y * intensity * 1.3,
        color.z * intensity * 1.3,
    )
}

pub fn red_planet_shader(fragment: &Fragment) -> Color {
    let light_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.3);
    
    let craters = ((fragment.tex_coords.x * 25.0).sin() * (fragment.tex_coords.y * 25.0).cos()).abs();
    let variation = 0.7 + craters * 0.3;
    
    Color::from_float(
        1.0 * intensity * variation,      // Más brillante
        0.4 * intensity * variation * 1.2,
        0.25 * intensity * variation * 1.2,
    )
}

// Shader para planeta helado - Más brillante y reflectivo
pub fn ice_planet_shader(fragment: &Fragment) -> Color {
    let light_dir = Vec3::new(1.0, 0.5, 0.5).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.3);  // Aumentado de 0.1 a 0.3
    
    let ice_pattern = ((fragment.tex_coords.x * 15.0).cos() * (fragment.tex_coords.y * 15.0).sin()).abs();
    
    let brightness = 0.9 + ice_pattern * 0.2;  // Más claro
    
    Color::from_float(
        brightness * intensity * 1.4,  // +40% brillo (hielo refleja más)
        brightness * intensity * 1.35,
        brightness * intensity * 1.4,
    )
}

pub fn moon_shader(fragment: &Fragment) -> Color {
    let light_dir = Vec3::new(1.0, 0.5, 0.5).normalize();
    let intensity = fragment.normal.dot(&light_dir).max(0.3);  // Mejorado
    
    let craters = ((fragment.tex_coords.x * 30.0).sin() * (fragment.tex_coords.y * 30.0).cos()).abs();
    let gray = 0.6 + craters * 0.3;  // Más claro
    
    Color::from_float(
        gray * intensity * 1.2,
        gray * intensity * 1.2,
        gray * intensity * 1.2,
    )
}
