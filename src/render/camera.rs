use nalgebra;
use nalgebra::{Point3, Vector3, Isometry3, Perspective3, UnitQuaternion, Translation3, Quaternion};

pub struct Camera {
    pub view: Isometry3<f32>,
}

impl Camera{
    pub fn new() -> Camera{
        let isometry = Isometry3::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,0.0));
        Camera{
            view: isometry
        }
    }
    pub fn set_pos(&mut self, pos: Vector3<f32>){
        self.view.translation = Translation3::from_vector(-pos);
    }
    pub fn set_rot(&mut self, rot: UnitQuaternion<f32>){
        self.view.rotation = rot;
    }
}
