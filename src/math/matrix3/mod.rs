use std::ops::{Mul};
use super::matrix2::{Matrix2};

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    pub data: [f32; 9],
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            data: [
                self.data[0] * rhs.data[0] + self.data[1] * rhs.data[3] + self.data[2] * rhs.data[6],   self.data[0] * rhs.data[1] + self.data[1] * rhs.data[4] + self.data[2] * rhs.data[7],   self.data[0] * rhs.data[2] + self.data[1] * rhs.data[5] + self.data[2] * rhs.data[8],
                self.data[3] * rhs.data[0] + self.data[4] * rhs.data[3] + self.data[5] * rhs.data[6],   self.data[3] * rhs.data[1] + self.data[4] * rhs.data[4] + self.data[5] * rhs.data[7],   self.data[3] * rhs.data[2] + self.data[4] * rhs.data[5] + self.data[5] * rhs.data[8],
                self.data[6] * rhs.data[0] + self.data[7] * rhs.data[3] + self.data[8] * rhs.data[6],   self.data[6] * rhs.data[1] + self.data[7] * rhs.data[4] + self.data[8] * rhs.data[7],   self.data[6] * rhs.data[2] + self.data[7] * rhs.data[5] + self.data[8] * rhs.data[8],
            ]
        }
    }
}

impl Mul<Matrix3> for f32 {
    type Output = Matrix3;
    
    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            data: [
                self * rhs.data[0], self * rhs.data[1], self * rhs.data[2],
                self * rhs.data[3], self * rhs.data[4], self * rhs.data[5],
                self * rhs.data[6], self * rhs.data[7], self * rhs.data[8],
            ]
        }
    }
}

impl Matrix3 {
    pub fn new() -> Matrix3 {
        Matrix3 {
            data: [
                1., 0., 0.,
                0., 1., 0.,
                0., 0., 1.,
            ]
        }
    }

    pub fn transpose(&self) -> Matrix3 {
        Matrix3 {
            data: [
                self.data[0], self.data[3], self.data[6],
                self.data[1], self.data[4], self.data[7],
                self.data[2], self.data[5], self.data[8],
            ]
        }
    }

    pub fn cofactor(&self) -> Matrix3 {
        let mut matrix = Matrix3::new();
        let minus_one: f32 = -1.;
        for i in 0..3 {
            for j in 0..3 {
                matrix.data[i + (j * 3)] = (minus_one.powi(i as i32 + 1 + j as i32 + 1)) * self.minor(i, j).determinant();
            }
        }
        
        matrix
    }

    pub fn determinant(&self) -> f32 {
        self.data[0] * self.minor(0, 0).determinant() - 
        self.data[1] * self.minor(1, 0).determinant() + 
        self.data[2] * self.minor(2, 0).determinant()
    }

    pub fn minor(&self, i: usize, j: usize) -> Matrix2 {
        assert!(i < 3);
        assert!(j < 3);
    
        let mut matrix = Matrix2::new();
        let indices = match i + (j * 3) {
            0 => [4, 5, 7, 8],
            1 => [3, 5, 6, 8],
            2 => [3, 4, 6, 7],
            3 => [1, 2, 7, 8],
            4 => [0, 2, 6, 8],
            5 => [0, 1, 6, 7],
            6 => [1, 2, 4, 5],
            7 => [0, 2, 3, 5],
            8 => [0, 1, 3, 4],
            _ => panic!("Invalid index"),
        };
        
        for i in 0..4 {
            matrix.data[i] = self.data[indices[i]];
        }
    
        matrix
        
    }
}