use conrod;
use glium;
use render;
use support;
use glium::{Display};
use glium::texture::Texture2d;
use conrod::{widget, Positionable, Widget, Colorable, Sizeable, Labelable};
// I still don't really understand how conrod works, so this code can be pretty terrible
widget_ids!(pub struct Ids { text, button, canvas, cursor });

pub struct Ui{
    pub ui: conrod::Ui,
    pub renderer: conrod::backend::glium::Renderer,
    pub image_map: conrod::image::Map<glium::texture::Texture2d>,
    pub ids: Ids,
    pub cursor_tex: conrod::image::Id
}
impl Ui{
    pub fn new(display: &Display, scr_res: (u32, u32)) -> Ui{
        let mut ui = conrod::UiBuilder::new([scr_res.0 as f64 , scr_res.1 as f64]).build();
        match ui.fonts.insert_from_file("./assets/fonts/Roboto-Medium.ttf"){
            Ok(_) => {},
            _ => println!("Failed to load font")
        };
        let renderer = conrod::backend::glium::Renderer::new(display).unwrap();
        let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
        let cursor_tex = image_map.insert(support::texture_loader::load("./assets/cursor.png".to_string(), display));
        let ids = Ids::new(ui.widget_id_generator());
        Ui{
            ui,
            renderer,
            image_map,
            ids,
            cursor_tex
        }
    }
    pub fn draw_into_texture(&mut self, display: &Display, res: (u32, u32)) -> Option<Texture2d>{
        use glium::framebuffer::SimpleFrameBuffer;
        use glium::texture::{DepthTexture2d, Texture2d, DepthFormat, UncompressedFloatFormat, MipmapsOption};
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&display, primitives, &self.image_map);

            let depthtexture = DepthTexture2d::empty_with_format(display, DepthFormat::F32, MipmapsOption::NoMipmap, res.0, res.1).unwrap();
            let area_tex = Texture2d::empty_with_format(display, UncompressedFloatFormat::F32F32F32F32, MipmapsOption::NoMipmap, res.0, res.1).unwrap();
            {
                let mut target = SimpleFrameBuffer::with_depth_buffer(display, &area_tex, &depthtexture).unwrap();
                self.renderer.draw(display, &mut target, &self.image_map).unwrap();
            }
            Some(area_tex)
        }
        else{
            None
        }
    }
    pub fn update(&mut self, window: &mut render::Window){
        for event in &window.events{
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &window.display) {
                self.ui.handle_event(event);
            }
        }

        let ui_w = &mut self.ui.set_widgets();

        widget::Canvas::new().color(conrod::color::rgba(0.0, 0.0, 0.0, 0.0)).pad(40.0).set(self.ids.canvas, ui_w);

        for _click in widget::Button::new()
            .middle_of(self.ids.canvas)
            .w_h(128.0, 128.0)
            .label("test")
            .label_font_size(32)
            .set(self.ids.button, ui_w)
        {
            println!("Test button was pressed");
        }

        widget::Image::new(self.cursor_tex).w_h(32.0, 32.0)
            .x_position(conrod::position::Position::Absolute(window.mouse_pos.0 as f64 - (window.scr_res.0 / 2) as f64 + 17.0))
            .y_position(conrod::position::Position::Absolute(-(window.mouse_pos.1 as f64) + (window.scr_res.1 / 2) as f64 - 17.0))
            .floating(true)
            .set(self.ids.cursor, ui_w);
    }
}
