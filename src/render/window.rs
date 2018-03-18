
use render::draw_display::DrawDisplay;
use glium::{glutin, Display};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};

#[derive(PartialEq)]
pub enum RenderMode{
    VR,
    Desktop
}

pub struct Window{
    pub events_loop: EventsLoop,
    pub display: DrawDisplay,
}

impl Window {
    pub fn new(sizex: u32, sizey: u32, title: &'static str, rend_mode: &RenderMode) -> Window{

        let events_loop = glutin::EventsLoop::new();

        let window = match rend_mode{
            &RenderMode::VR => WindowBuilder::new()
            .with_dimensions(sizex, sizey)
            .with_title(title)
            .with_fullscreen(events_loop.get_available_monitors().last()),
            &RenderMode::Desktop => WindowBuilder::new()
            .with_dimensions(sizex, sizey)
            .with_title(title)
        };
        let context = ContextBuilder::new()
            .with_depth_buffer(24);

        let display = Display::new(window, context, &events_loop).unwrap();

        Window{
            events_loop: events_loop,
            display: DrawDisplay{display},
        }
    }

    pub fn get_display(&mut self) -> (&mut DrawDisplay, &mut EventsLoop){
        (&mut self.display, &mut self.events_loop)
    }
}
