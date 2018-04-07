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
        let mut pos = pos;
        pos[1] = -pos[1];
        self.view.translation = Translation3::from_vector(pos);
    }
    pub fn set_rot(&mut self, rot: (f32, f32, f32, f32)){
        self.view.rotation = UnitQuaternion::from_quaternion(Quaternion::new(rot.0, rot.1, rot.2, rot.3));
    }
}
