pub mod obj_loader;
pub mod texture_loader;
use rand::{random, thread_rng, Rng};
use nalgebra::{Translation3, UnitQuaternion, Vector3};
pub fn rand_string(len: u32) -> String {
    (0..len).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

pub fn rand_translation(range_x_z: (f32, f32), range_y: (f32, f32)) -> Translation3<f32> {
    let mut rng = thread_rng();
    let x = rng.gen_range(range_x_z.0, range_x_z.1);
    let y = rng.gen_range(range_y.0, range_y.1);
    let z = rng.gen_range(range_x_z.0, range_x_z.1);
    Translation3::new(x, y, z)
}

pub fn direction(rotation: UnitQuaternion<f32>, vec: Vector3<f32>) -> Vector3<f32>{
    use alga::linear::Transformation;
    let mut direction = vec;
    let matrix = rotation.to_homogeneous();
    direction = matrix.transform_vector(&direction);
    direction
}
