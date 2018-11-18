use nalgebra::geometry::{Perspective3, UnitQuaternion, Quaternion, Translation3};
use nalgebra::{Matrix4};

pub struct CharacterView{
    pub position: Translation3<f32>,
    pub rotation: UnitQuaternion<f32>
}

pub struct Camera{
    pub view: Matrix4<f32>,
    pub perspective: Matrix4<f32>
}

impl Camera{
    pub fn new(sx: u32, sy: u32) -> Camera{
        let perspective = Perspective3::new(sx as f32 / sy as f32, 3.14 / 2.5, 0.01, 200000.0).to_homogeneous();

        let position = Translation3::new(0.0,0.0,0.0) * UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0));

        Camera{
            view: position.to_homogeneous(),
            perspective
        }
    }
    /*pub fn set_view(&mut self, position: Translation3<f32>, rotation: UnitQuaternion<f32>){
        self.view = rotation.to_homogeneous() * position.to_homogeneous();
    }*/
}

impl CharacterView{
    pub fn new() -> CharacterView{
        CharacterView{
            position: Translation3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 1.0, 0.0)),
        }
    }
    pub fn calc_view(&self) -> Matrix4<f32>{
        let mut position = self.position;
        position.vector *= -1.0;
        let translation_matrix = position.to_homogeneous();
        let rotation_matrix = self.rotation.to_homogeneous();
        rotation_matrix * translation_matrix
    }
}
