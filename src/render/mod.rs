use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion, Translation3};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder, dpi};
use nalgebra::core::{Matrix4, MatrixN};
use glium::{glutin, Display, Program};
use std::collections::HashMap;
use openhmd_rs;

pub mod default_shaders;
pub mod camera;
pub mod model;
pub mod draw;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: [f32; 2],
}
implement_vertex!(Vertex2D, position);

pub struct DrawArea{
    camera: camera::Camera,
    res: (u32, u32),
    size: (f32, f32),
    pos: (f32, f32),
    distortion_shader: bool
}

pub struct Window{
    pub display: Display,
    pub events_loop: EventsLoop,
    pub draw_areas: HashMap<String, DrawArea>,
    pub shaders: HashMap<&'static str, Program>,
    pub draw_buffer: draw::DrawBuffer,
    pub ohmd_context: Option<openhmd_rs::Context>,
    pub ohmd_device: Option<openhmd_rs::Device>,
    pub character_view: camera::CharacterView
}

impl Window{
    pub fn new(x_size: u32, y_size: u32, title: &'static str, vr: bool) -> Window{
        let window = WindowBuilder::new()
            .with_dimensions(dpi::LogicalSize::new(x_size as f64, y_size as f64))
            .with_title(title);

        let context = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(false);

        let events_loop = glutin::EventsLoop::new();

        let display = Display::new(window, context, &events_loop).unwrap();

        let draw_areas = HashMap::with_capacity(128);

        let ohmd_context =
        if vr {
            Some(openhmd_rs::Context::new())
        }
        else{
            None
        };

        let ohmd_device =
        if let &Some(ref ohmd_context) = &ohmd_context{
            ohmd_context.probe();
            ohmd_context.update();
            Some(ohmd_context.list_open_device(0))
        }
        else{
            None
        };

        Window{
            display,
            events_loop,
            draw_areas,
            shaders: HashMap::with_capacity(128),
            draw_buffer: draw::DrawBuffer{
                objects: vec![]
            },
            ohmd_context,
            ohmd_device,
            character_view: camera::CharacterView::new()
        }
    }
    pub fn init(&mut self){
        println!("Loading shaders...");
        let camera = camera::Camera::new(1024, 768);
        self.add_shader("simple", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SIMPLE_FRAG);
        self.add_shader("solid", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_shader("solid_2d", default_shaders::SHADER_SIMPLE_2D_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_draw_area("default".to_string(), camera, (1024, 768), (2.0, 2.0), (-1.0, -1.0), false);
    }
    pub fn init_vr(&mut self, hmd_x: u32, hmd_y: u32){
        println!("Loading shaders...");
        let mut camera = camera::Camera::new(hmd_x / 2, hmd_y);
        let mut camera2 = camera::Camera::new(hmd_x / 2, hmd_y);
        if let &mut Some(ref mut ohmd_device) = &mut self.ohmd_device{
            let view_l: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_view_matrix_l());
            let view_r: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_view_matrix_r());

            let proj_l: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_proj_matrix_l());
            let proj_r: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_proj_matrix_r());

            camera.view = view_l;
            camera2.view = view_r;

            camera.perspective = proj_l;
            camera2.perspective = proj_r;
        }
        self.add_shader("simple", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SIMPLE_FRAG);
        self.add_shader("solid", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_shader("solid_2d", default_shaders::SHADER_SIMPLE_2D_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_draw_area("vr_cam_left".to_string(), camera, (hmd_x / 2, hmd_y), (1.0, 2.0), (-1.0, -1.0), false);
        self.add_draw_area("vr_cam_right".to_string(), camera2, (hmd_x / 2, hmd_y), (1.0, 2.0), (0.0, -1.0), false);
    }
    pub fn update_vr(&mut self){
        if let &mut Some(ref mut ohmd_context) = &mut self.ohmd_context{
            ohmd_context.update();
        }
        if let &mut Some(ref mut ohmd_device) = &mut self.ohmd_device{
            println!("test");
            let chrctr_view = self.character_view.calc_view();
            {
                let eye_left = self.draw_areas.get_mut("vr_cam_left").unwrap();
                let proj_l: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_proj_matrix_l());
                let view: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_view_matrix_l());
                eye_left.camera.view = view * chrctr_view;
                eye_left.camera.perspective = proj_l;
                println!("{:?}", eye_left.camera.perspective);
            }
            {
                let eye_right = self.draw_areas.get_mut("vr_cam_right").unwrap();
                let view: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_view_matrix_r());
                let proj_r: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_proj_matrix_r());
                eye_right.camera.view = view * chrctr_view;
                eye_right.camera.perspective = proj_r;
            }
        }
    }
}

pub fn mat16_to_nalg(mat: [f32;16]) ->Matrix4<f32>{
    let mut raw: Matrix4<f32> = MatrixN::new_scaling(0.0);
    for x in 0..16{
        raw[x] = mat[x];
    }
    raw
}
