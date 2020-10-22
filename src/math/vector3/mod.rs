use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;
    
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }

}

impl Mul for Vector3 {
    type Output = Self;
    
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z:f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    } 

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vector3 {
        Vector3 {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    pub fn normal(self, a: Vector3, b: Vector3) -> Vector3 {
        let u: Vector3 = a - self;
        let v: Vector3 = b - self;
        u.cross(v).normalize()
    }

    pub fn cross(self, a: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * a.z - self.z * a.y,
            y: self.z * a.x - self.x * a.z,
            z: self.x * a.y - self.y * a.z,
        }
    }

    pub fn dot(self, a: Vector3) -> f32 {
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}