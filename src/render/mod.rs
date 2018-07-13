use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder, dpi};
use glium::{glutin, Display, Program};
use std::collections::HashMap;

pub mod draw;
pub mod mesh;
pub mod default_shaders;

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
    res: (u32, u32),
    size: (f32, f32),
    pos: (f32, f32),
    distortion_shader: bool
}

pub struct Window{
    pub display: Display,
    pub events_loop: EventsLoop,
    pub draw_areas: Vec<DrawArea>,
    pub shaders: HashMap<&'static str, Program>,
    pub draw_buffer: draw::Draw_Buffer
}

impl Window{
    pub fn new(x_size: u32, y_size: u32, title: &'static str) -> Window{
        let window = WindowBuilder::new()
            .with_dimensions(dpi::LogicalSize::new(x_size as f64, y_size as f64))
            .with_title(title);

        let context = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(false);

        let events_loop = glutin::EventsLoop::new();

        let display = Display::new(window, context, &events_loop).unwrap();

        let draw_areas = vec![];

        Window{
            display,
            events_loop,
            draw_areas,
            shaders: HashMap::with_capacity(128),
            draw_buffer: draw::Draw_Buffer{
                models: vec![]
            }
        }
    }
    pub fn init(&mut self){
        println!("Loading shaders...");
        self.add_shader("simple", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SIMPLE_FRAG);
        self.add_shader("solid", default_shaders::SHADER_SIMPLE_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_shader("solid_2d", default_shaders::SHADER_SIMPLE_2D_VERT, default_shaders::SHADER_SOLID_FRAG);
        self.add_draw_area((1920, 1080), (2.0, 2.0), (-1.0, -1.0), false);
    }
}
