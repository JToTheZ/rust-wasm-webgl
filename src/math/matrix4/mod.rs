use std::ops::{Mul};
use super::matrix3::{Matrix3};
use super::vector3::{Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    pub data: [f32; 16],
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        Matrix4 {
            data:
            [
                 self.data[0] * rhs.data[0] +  self.data[1] * rhs.data[4] +  self.data[2] * rhs.data[8] +  self.data[3] * rhs.data[12],      self.data[0] * rhs.data[1] + self.data[1] * rhs.data[5] + self.data[2] * rhs.data[9] + self.data[3] * rhs.data[13],        self.data[0] * rhs.data[2] + self.data[1] * rhs.data[6] + self.data[2] * rhs.data[10] + self.data[3] * rhs.data[14],       self.data[0] * rhs.data[3] + self.data[1] * rhs.data[7] + self.data[2] * rhs.data[11] + self.data[3] * rhs.data[15],
                 self.data[4] * rhs.data[0] +  self.data[5] * rhs.data[4] +  self.data[6] * rhs.data[8] +  self.data[7] * rhs.data[12],      self.data[4] * rhs.data[1] + self.data[5] * rhs.data[5] + self.data[6] * rhs.data[9] + self.data[7] * rhs.data[13],        self.data[4] * rhs.data[2] + self.data[5] * rhs.data[6] + self.data[6] * rhs.data[10] + self.data[7] * rhs.data[14],       self.data[4] * rhs.data[3] + self.data[5] * rhs.data[7] + self.data[6] * rhs.data[11] + self.data[7] * rhs.data[15],
                 self.data[8] * rhs.data[0] +  self.data[9] * rhs.data[4] + self.data[10] * rhs.data[8] + self.data[11] * rhs.data[12],      self.data[8] * rhs.data[1] + self.data[9] * rhs.data[5] + self.data[10] * rhs.data[9] + self.data[11] * rhs.data[13],      self.data[8] * rhs.data[2] + self.data[9] * rhs.data[6] + self.data[10] * rhs.data[10] + self.data[11] * rhs.data[14],     self.data[8] * rhs.data[3] + self.data[9] * rhs.data[7] + self.data[10] * rhs.data[11] + self.data[11] * rhs.data[15],
                self.data[12] * rhs.data[0] + self.data[13] * rhs.data[4] + self.data[14] * rhs.data[8] + self.data[15] * rhs.data[12],     self.data[12] * rhs.data[1] + self.data[13] * rhs.data[5] + self.data[14] * rhs.data[9] + self.data[15] * rhs.data[13],    self.data[12] * rhs.data[2] + self.data[13] * rhs.data[6] + self.data[14] * rhs.data[10] + self.data[15] * rhs.data[14],   self.data[12] * rhs.data[3] + self.data[13] * rhs.data[7] + self.data[14] * rhs.data[11] + self.data[15] * rhs.data[15],
            ]
        }
    }
}

impl Mul<Matrix4> for f32 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        Matrix4 {
            data:
            [
                  self * rhs.data[0], self * rhs.data[1],   self * rhs.data[2], self * rhs.data[3],
                  self * rhs.data[4], self * rhs.data[5],   self * rhs.data[6], self * rhs.data[7],
                  self * rhs.data[8], self * rhs.data[9], self * rhs.data[10], self * rhs.data[11],
                self * rhs.data[12], self * rhs.data[13], self * rhs.data[14], self * rhs.data[15],
            ]
        }
    }
}



impl Matrix4 {
    pub fn new() -> Matrix4 {
        Matrix4 {
            data: [
                1., 0., 0., 0.,
                0., 1., 0., 0.,
                0., 0., 1., 0.,
                0., 0., 0., 1.,
            ]
        }
    }

    pub fn transpose(&self) -> Matrix4 {
        Matrix4 {
            data: [ 
                self.data[0], self.data[4],  self.data[8], self.data[12],
                self.data[1], self.data[5],  self.data[9], self.data[13],
                self.data[2], self.data[6], self.data[10], self.data[14],
                self.data[3], self.data[7], self.data[11], self.data[15],
            ]
        }
    }

    pub fn cofactor(&self) -> Matrix4 {
        let mut matrix = Matrix4::new();
        let minus_one: f32 = -1.;
        for i in 0..4 {
            for j in 0..4 {
                matrix.data[i + (j * 4)] = (minus_one.powi(i as i32 + 1 + j as i32 + 1)) * self.minor(i, j).determinant();
            }
        }
        
        matrix
    }

    pub fn inverse(&self) -> Result<Matrix4, &'static str> {
        let determinant = self.determinant();
        if determinant == 0. {
            Err("Matrix does not have an inverse")
        } else {
            Ok(1. / determinant * self.cofactor().transpose())
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0] * self.minor(0, 0).determinant() -
        self.data[1] * self.minor(1, 0).determinant() +
        self.data[2] * self.minor(2, 0).determinant() -
        self.data[3] * self.minor(3, 0).determinant()
    }

    pub fn minor(&self, i: usize, j: usize) -> Matrix3 {
        assert!(i < 4);
        assert!(j < 4);
    
        let mut matrix = Matrix3::new();
        let indices = match i + (j * 4) {
            0 => [5, 6, 7, 9, 10, 11, 13, 14, 15],
            1 => [4, 6, 7, 8, 10, 11, 12, 14, 15],
            2 => [4, 5, 7, 8, 9, 10, 12, 13, 15],
            3 => [4, 5, 6, 8, 9, 10, 12, 13, 14],
    
            4 => [1, 2, 3, 9, 10, 11, 13, 14, 15],
            5 => [0, 2, 3, 8, 10, 11, 12, 14, 15],
            6 => [0, 1, 3, 8, 9, 11, 12, 13, 15],
            7 => [0, 1, 2, 8, 9, 10, 12, 13, 14],
    
            8 => [1, 2, 3, 5, 6, 7, 13, 14, 15],
            9 => [0, 2, 3, 4, 6, 7, 12, 14, 15],
            10 => [0, 1, 3, 4, 5, 7, 12, 13, 15],
            11 => [0, 1, 2, 4, 5, 6, 12, 13, 14],
    
            12 => [1, 2, 3, 5, 6, 7, 9, 10, 11],
            13 => [0, 2, 3, 4, 6, 7, 8, 10, 11],
            14 => [0, 1, 3, 4, 5, 7, 8, 9, 11],
            15 => [0, 1, 2, 4, 5, 6, 8, 9, 10],
            _ => panic!("Invalid index"),
        };
    
        for i in 0..9 {
            matrix.data[i] = self.data[indices[i]];
        }
    
        matrix

    }
}


pub fn perspective_matrix(aspect: f32, fov_angle: f32, z_near: f32, z_far: f32) -> Matrix4 {
    let scale = (fov_angle * 0.5 * std::f32::consts::PI / 180.) * z_near;
    let right = aspect * scale;
    let top = scale;

    Matrix4 {
        data: [
            z_near / right,           0.,                                    0.,  0.,
                        0., z_near / top,                                    0.,  0.,
                        0.,           0.,    -(z_far + z_near) / z_far - z_near, -1.,
                        0.,           0., -2. * z_far * z_near / z_far - z_near,  0.,
        ]
    }
}

pub fn rotation_matrix_x(theta: f32) -> Matrix4 {
    Matrix4 {
        data:[
            1.,            0.,           0., 0.,
            0.,   theta.cos(), -theta.sin(), 0.,
            0.,   theta.sin(),  theta.cos(), 0., 
            0.,            0.,           0., 1.,
        ]
    }
}

pub fn rotation_matrix_y(theta: f32) -> Matrix4 {
    Matrix4 {
        data: [
             theta.cos(), 0.,  theta.sin(), 0.,
                      0., 1.,           0., 0.,
            -theta.sin(), 0.,  theta.cos(), 0., 
                      0., 0.,           0., 1.,
        ]
    }
}

pub fn rotation_matrix_z(theta: f32) -> Matrix4 {
    Matrix4 {
        data: [
            theta.cos(), -theta.sin(), 0., 0.,
            theta.sin(),  theta.cos(), 0., 0.,
                     0.,           0., 1., 0., 
                     0.,           0., 0., 1.,
        ]
    }
}

pub fn translation_matrix_xyz(x: f32, y: f32, z: f32) -> Matrix4 {
    Matrix4 {
        data: [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
             x,  y,  z, 1.,
        ]
    }
}

pub fn scale_matrix_xyz(x: f32, y: f32, z: f32) -> Matrix4 {
    Matrix4 {
        data: [
            x,  0., 0., 0.,
            0.,  y, 0., 0.,
            0., 0.,  z, 0.,
            0., 0., 0., 1.,
        ]
    }
}

pub fn look_at(eye: Vector3, at: Vector3, up: Vector3) -> Matrix4 {
    let mut z_axis = (at - eye).normalize();
    let x_axis = (z_axis.cross(up)).normalize();
    let y_axis = x_axis.cross(z_axis);
    z_axis.negate();
    Matrix4 {
        data: [
            x_axis.x, x_axis.y, x_axis.z, -1. * x_axis.dot(eye),
            y_axis.x, y_axis.y, y_axis.z, -1. * y_axis.dot(eye),
            z_axis.x, z_axis.y, z_axis.z, -1. * z_axis.dot(eye),
                  0.,       0.,       0.,               1.,
        ]
    }
}

pub fn fps_camera(eye: Vector3, pitch: f32, yaw: f32) -> Matrix4 {
    let x_axis = Vector3::new(yaw.cos(), 0., -1. * yaw.sin());
    let y_axis = Vector3::new(yaw.sin() * pitch.sin(), pitch.cos(), yaw.cos() * pitch.sin());
    let z_axis = Vector3::new(yaw.sin() * pitch.cos(), -1. * pitch.sin(), pitch.cos() * yaw.cos());

    Matrix4 {
        data: [
            x_axis.x, x_axis.y, x_axis.z, -1. * (x_axis.dot(eye)),
            y_axis.x, y_axis.y, y_axis.z, -1. * (y_axis.dot(eye)),
            z_axis.x, z_axis.y, z_axis.z, -1. * (z_axis.dot(eye)),
                  0.,       0.,       0.,                      1.,
        ]
    }
}