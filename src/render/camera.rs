
use nalgebra::geometry::{Perspective3, UnitQuaternion, Quaternion, Translation3, Point3};
use nalgebra::{Matrix4};
pub struct Camera{
    pub view: Matrix4<f32>,
    pub perspective: Matrix4<f32>
}

impl Camera{
    pub fn new(sx: u32, sy: u32) -> Camera{
        let perspective = Perspective3::new(sx as f32 / sy as f32, 3.14 / 2.0, 0.01, 200000.0).to_homogeneous();

        let position = Translation3::new(0.0,0.0,0.0) * UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0));

        Camera{
            view: position.to_homogeneous(),
            perspective
        }
    }
    pub fn set_view(&mut self, position: Translation3<f32>, rotation: UnitQuaternion<f32>){
        self.view = rotation.to_homogeneous() * position.to_homogeneous();
    }
}
