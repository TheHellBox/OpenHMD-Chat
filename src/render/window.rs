
use render::draw_display::Draw_Display;
use glium::{glutin, Display};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};

pub struct Window{
    pub events_loop: EventsLoop,
    display: Draw_Display
}

impl Window {
    pub fn new(sizex: u32, sizey: u32, title: &'static str) -> Window{
        let window = WindowBuilder::new()
            .with_dimensions(sizex, sizey)
            .with_title(title);

        let mut events_loop = glutin::EventsLoop::new();

        let context = ContextBuilder::new()
            .with_depth_buffer(24);

        let display = Display::new(window, context, &events_loop).unwrap();

        Window{
            events_loop: events_loop,
            display: Draw_Display{display}
        }
    }

    pub fn get_display(&mut self) -> (&mut Draw_Display, &mut EventsLoop){
        (&mut self.display, &mut self.events_loop)
    }
}
