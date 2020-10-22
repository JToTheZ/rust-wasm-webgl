#[derive(Clone, Debug)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3D {
    pub fn new() -> Self {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl From<[f32; 3]> for Point3D {
    fn from(value: [f32;3]) -> Self {
        Point3D {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}