use nalgebra::{Point3, Vector3, Isometry3, Perspective3, Matrix4, Rotation3,UnitQuaternion, Unit, Translation3};

pub struct Camera {
    pub view: Isometry3<f32>,
    pub persp: Perspective3<f32>
}

impl Camera{
    pub fn new() -> Camera{
        let mut persp = Perspective3::new(960.0/1080.0, 3.14 / 4.0, 0.1, 10000.0);

        let mut view = Isometry3::look_at_rh(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0,1.0,0.0));
        view.translation = Translation3::new(0.0,0.0,0.0);
        Camera{
            view: view,
            persp: persp
        }
    }
    pub fn set_rot(&mut self, rot: UnitQuaternion<f32>){

        self.view.rotation = rot;
    }
}

pub struct Eyes {
    pub cam1: Camera,
    pub cam2: Camera
}

impl Eyes{
    pub fn new() -> Eyes{
        let mut cam1 = Camera::new();
        let mut cam2 = Camera::new();
        cam1.view.translation = Translation3::new(0.0,0.0,0.0);
        cam2.view.translation = Translation3::new(0.0,0.0,0.0);
        Eyes{
            cam1: cam1,
            cam2: cam2
        }
    }
    pub fn set_rot(&mut self, rot: UnitQuaternion<f32>){
        self.cam1.view.rotation = rot;
        self.cam2.view.rotation = rot;
    }
}
