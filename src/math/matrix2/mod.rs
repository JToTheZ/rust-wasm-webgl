use std::ops::{Mul};

#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    pub data: [f32; 4],
}

impl Mul for Matrix2 {
    type Output = Self;

    fn mul(self, rhs: Matrix2) -> Self::Output {
        Matrix2 {
            data:[
                self.data[0] * rhs.data[0] + self.data[1] * rhs.data[2], self.data[0] * rhs.data[2] + self.data[1] * rhs.data[3],
                self.data[2] * rhs.data[0] + self.data[3] * rhs.data[2], self.data[2] * rhs.data[0] + self.data[3] * rhs.data[1],
            ]
        }
    }
}

impl Mul<Matrix2> for f32 {
    type Output = Matrix2;
    
    fn mul(self, rhs: Matrix2) -> Self::Output {
        Matrix2 {
            data: [
                self * rhs.data[0], self * rhs.data[1],
                self * rhs.data[2], self * rhs.data[3],
            ]
        }
    }
}

impl Matrix2 {
    pub fn new() -> Matrix2 {
        Matrix2 {
            data: [
                1., 0.,
                0., 1.,
            ]
        }
    }

    pub fn transpose(&self) -> Matrix2 {
        Matrix2 {
            data: [
                self.data[0], self.data[2],
                self.data[1], self.data[3],
            ]
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }
}