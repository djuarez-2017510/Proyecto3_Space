use crate::math::{Vec3, Mat4, look_at};

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
    pub angle: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let angle = 0.0;
        let distance = (eye - center).magnitude();
        
        Camera {
            eye,
            center,
            up,
            has_changed: true,
            angle,
            distance,
        }
    }

    pub fn orbit(&mut self, delta_angle: f32, _vertical: f32) {
        self.angle += delta_angle;
        
        self.eye.x = self.center.x + self.distance * self.angle.cos();
        self.eye.z = self.center.z + self.distance * self.angle.sin();
        
        self.has_changed = true;
    }

    pub fn orbit_around_center(&mut self, delta_angle: f32) {
        self.orbit(delta_angle, 0.0);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance + delta).max(5.0).min(100.0);
        
        self.eye.x = self.center.x + self.distance * self.angle.cos();
        self.eye.z = self.center.z + self.distance * self.angle.sin();
        
        self.has_changed = true;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(&self.eye, &self.center, &self.up)
    }

    pub fn move_center(&mut self, direction: Vec3) {
        self.center += direction;
        self.eye += direction;
        self.has_changed = true;
    }
}
