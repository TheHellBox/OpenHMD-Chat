pub mod obj_loader;
pub mod texture_loader;
use rand::{random};

pub fn rand_string(len: u32) -> String {
    (0..len).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

/*pub fn random_number() -> u32{
    let mut rng = thread_rng();
    rng.gen_range(0, 999999)
}

pub fn direction(rotation: UnitQuaternion<f32>, vec: Vector3<f32>) -> Vector3<f32>{
    use alga::linear::Transformation;
    let mut direction = vec;
    let matrix = rotation.to_homogeneous();
    direction = matrix.transform_vector(&direction);
    direction
}*/
