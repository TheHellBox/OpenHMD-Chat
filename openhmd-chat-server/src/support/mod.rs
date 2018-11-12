use rand::random;
use nalgebra::{UnitQuaternion, Vector3};

pub fn rand_string(len: u32) -> String {
    (0..len).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

pub fn direction(rotation: UnitQuaternion<f32>, vec: Vector3<f32>) -> Vector3<f32>{
    let mut direction = vec;
    let matrix = rotation.to_homogeneous();
    direction = matrix.transform_vector(&direction);
    direction
}
