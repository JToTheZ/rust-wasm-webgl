#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4 {
    pub fn new() -> Vector4 {
        Vector4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(self) -> Vector4 {
        Vector4 {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
            w: self.w / self.length(),
        }
    } 
}