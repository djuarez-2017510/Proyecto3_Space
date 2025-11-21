use std::ops::{Add, Sub, Mul, Neg, AddAssign};

// ============ Vec3 ============
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zeros() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vec3::new(self.x / mag, self.y / mag, self.z / mag)
        } else {
            *self
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(vec.x * self, vec.y * self, vec.z * self)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// ============ Vec4 ============
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }
}

// ============ Mat4 ============
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Self {
        Mat4 {
            data: [
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33],
            ],
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, vec: Vec4) -> Vec4 {
        Vec4::new(
            self.data[0][0] * vec.x + self.data[0][1] * vec.y + self.data[0][2] * vec.z + self.data[0][3] * vec.w,
            self.data[1][0] * vec.x + self.data[1][1] * vec.y + self.data[1][2] * vec.z + self.data[1][3] * vec.w,
            self.data[2][0] * vec.x + self.data[2][1] * vec.y + self.data[2][2] * vec.z + self.data[2][3] * vec.w,
            self.data[3][0] * vec.x + self.data[3][1] * vec.y + self.data[3][2] * vec.z + self.data[3][3] * vec.w,
        )
    }
}

// ============ Transformaciones ============

pub fn translate(translation: &Vec3) -> Mat4 {
    Mat4::new(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn scale(scale: &Vec3) -> Mat4 {
    Mat4::new(
        scale.x, 0.0, 0.0, 0.0,
        0.0, scale.y, 0.0, 0.0,
        0.0, 0.0, scale.z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn rotate_x(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, c, -s, 0.0,
        0.0, s, c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn rotate_y(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::new(
        c, 0.0, s, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -s, 0.0, c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn rotate_z(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::new(
        c, -s, 0.0, 0.0,
        s, c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn look_at(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4 {
    let f = (*center - *eye).normalize();
    let s = f.cross(up).normalize();
    let u = s.cross(&f);

    Mat4::new(
        s.x, s.y, s.z, -s.dot(eye),
        u.x, u.y, u.z, -u.dot(eye),
        -f.x, -f.y, -f.z, f.dot(eye),
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let tan_half_fov = (fov / 2.0).tan();
    
    Mat4::new(
        1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_half_fov, 0.0, 0.0,
        0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near),
        0.0, 0.0, -1.0, 0.0,
    )
}

pub fn viewport(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}
