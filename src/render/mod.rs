use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder, Event};
use nalgebra::core::{Matrix4, MatrixN};
use glium::glutin::Event::WindowEvent;
use glium::{glutin, Display, Program};
use std::collections::HashMap;
use game::settings::Settings;
use nalgebra::{UnitQuaternion, Quaternion};
use openhmd_rs;
use ui;

pub mod default_shaders;
pub mod ohmd_params;
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
    pub hmd_params: Option<ohmd_params::HMDParams>,
    pub character_view: camera::CharacterView,
    pub scr_res: (u32, u32),
    pub mouse_pos: (u32, u32),
    pub head_dir: UnitQuaternion<f32>,
    pub events: Vec<Event>,
    pub ui: ui::Ui
}

impl Window{
    pub fn new(title: &'static str, settings: &Settings) -> Window{

        let mut x_size = settings.screen_resx;
        let mut y_size = settings.screen_resy;

        let ohmd_context =
            if settings.vr_mode {
                Some(openhmd_rs::Context::new())
            }
            else{
                None
            };

        let ohmd_device =
            if let &Some(ref ohmd_context) = &ohmd_context{
                ohmd_context.probe();
                ohmd_context.update();
                let device = ohmd_context.list_open_device(0);
                x_size = device.get_scr_res_w();
                y_size = device.get_scr_res_h();
                Some(device)
            }
            else{
                None
            };
        let hmd_params =
            if let &Some(ref ohmd_device) = &ohmd_device{
                Some(ohmd_params::gen_ohmd_params(ohmd_device))
            }
            else{
                None
            };
        let title = if ohmd_context.is_some(){
            format!("{}: VR", title)
        }
        else{
            format!("{}: Desktop", title)
        };

        let events_loop = glutin::EventsLoop::new();

        let mut window = WindowBuilder::new()
            .with_dimensions(x_size, y_size)
            .with_title(title);

        if settings.full_screen {
            let monitor = events_loop.get_available_monitors().nth(settings.output_display as usize);
            window = window.with_fullscreen(monitor);
        }

        let context = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(false);


        let display = Display::new(window, context, &events_loop).unwrap();
        let _ = display.gl_window().window().set_cursor_state(glutin::CursorState::Hide);

        let draw_areas = HashMap::with_capacity(128);

        let ui = ui::Ui::new(&display, (x_size, y_size));

        Window{
            display,
            events_loop,
            draw_areas,
            shaders: HashMap::with_capacity(128),
            draw_buffer: draw::DrawBuffer{
                objects: HashMap::new()
            },
            ohmd_context,
            ohmd_device,
            hmd_params,
            character_view: camera::CharacterView::new(),
            scr_res: (x_size, y_size),
            mouse_pos: (0, 0),
            head_dir: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            events: vec![],
            ui
        }
    }
    pub fn is_vr(&self) -> bool{
        self.ohmd_device.is_some()
    }
    pub fn init(&mut self){
        println!("Loading shaders...");
        self.add_shader("simple", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SIMPLE_FRAG);
        self.add_shader("solid", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_shader("solid_2d", default_shaders::SHADER_SIMPLE_2D_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_shader("distortion_correction", default_shaders::SHADER_DISTORTION_VERT, default_shaders::SHADER_DISTORTION_FRAG);

        let scr_w = self.scr_res.0;
        let scr_h = self.scr_res.1;

        let camera = camera::Camera::new(scr_w, scr_h);
        if !self.is_vr() {
            self.add_draw_area("default".to_string(), camera, (scr_w, scr_h), (2.0, 2.0), (-1.0, -1.0), false);
        }
        else{
            self.init_vr(scr_w, scr_h);
        }
    }
    pub fn init_vr(&mut self, hmd_x: u32, hmd_y: u32){
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
        self.add_draw_area("vr_cam_left".to_string(), camera, (hmd_x / 2, hmd_y), (1.0, 2.0), (-1.0, -1.0), true);
        self.add_draw_area("vr_cam_right".to_string(), camera2, (hmd_x / 2, hmd_y), (1.0, 2.0), (0.0, -1.0), true);
    }
    pub fn update(&mut self){
        let mut events = vec![];
        let mut mouse_pos = self.mouse_pos;
        self.events_loop.poll_events(|event| {
            events.push(event.clone());
            //let _ = net_tx.send(NetworkEvent::SendRotation(window.head_dir));
            match event{
                WindowEvent { ref event, .. } => match event{
                    &glutin::WindowEvent::CursorMoved{position, ..} => {
                        mouse_pos = (position.0 as u32, position.1 as u32);
                    },
                    _ => {}
                },
                _ => {}
            }
        });
        self.events = events;
        self.mouse_pos = mouse_pos;
        let mut head_rotation = self.character_view.rotation.inverse();
        if self.is_vr(){
            self.update_vr();
            if let &mut Some(ref mut ohmd_device) = &mut self.ohmd_device{
                let rotation = ohmd_device.get_rotation_quat();
                let rotation = UnitQuaternion::from_quaternion(Quaternion::new(rotation[3], rotation[0], rotation[1], rotation[2]));
                head_rotation *= rotation;
            }
        }
        else{
            match self.draw_areas.get_mut("default"){
                Some(x) => {
                    x.camera.view = self.character_view.calc_view();
                },
                None => {}
            };
        }
        self.head_dir = head_rotation;
    }
    pub fn update_vr(&mut self){
        if let &mut Some(ref mut ohmd_context) = &mut self.ohmd_context{
            ohmd_context.update();
        }
        if let &mut Some(ref mut ohmd_device) = &mut self.ohmd_device{
            let chrctr_view = self.character_view.calc_view();
            {
                let eye_left = self.draw_areas.get_mut("vr_cam_left").unwrap();
                let proj_l: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_proj_matrix_l());
                let view: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_view_matrix_l()) * chrctr_view;
                eye_left.camera.view = view;
                eye_left.camera.perspective = proj_l;
            }
            {
                let eye_right = self.draw_areas.get_mut("vr_cam_right").unwrap();
                let proj_r: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_proj_matrix_r());
                let view: Matrix4<f32> = mat16_to_nalg(ohmd_device.get_view_matrix_r()) * chrctr_view;
                eye_right.camera.view = view;
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
pub fn m16_to_4x4(mat: [f32; 16]) -> [[f32;4]; 4]{
    let mat = [
        [mat[0], mat[1], mat[2], mat[3]],
        [mat[4], mat[5], mat[6], mat[7]],
        [mat[8], mat[9], mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ];
    mat
}
