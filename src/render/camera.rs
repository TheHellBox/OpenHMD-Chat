use nalgebra::{Point3, Vector3, Isometry3, Perspective3, UnitQuaternion, Translation3};

pub struct Camera {
    pub view: Translation3<f32>,
}

impl Camera{
    pub fn new() -> Camera{
        let translation = Translation3::new(0.0,0.0,0.0);
        Camera{
            view: translation
        }
    }
    pub fn set_pos(&mut self, pos: Vector3<f32>){
        self.view.vector = pos;
    }
}
