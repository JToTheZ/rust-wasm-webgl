#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x,
            y: y,
        }
    } 

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Vector2 {
        Vector2 {
            x: self.x / self.length(),
            y: self.y / self.length(),
        }
    }
}