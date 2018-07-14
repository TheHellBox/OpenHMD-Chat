use glium::{ Surface, VertexBuffer, IndexBuffer, Program, DrawParameters};
use glium::index::PrimitiveType;
use render::{Vertex2D, DrawArea, Window};
use render::model::Model;

pub struct Draw_Object{
    model: Model
}

pub struct Draw_Buffer{
    pub models: Vec<Draw_Object>
}

impl Window{
    pub fn draw(&mut self){
        use glium::framebuffer::SimpleFrameBuffer;
        use glium::texture::{DepthTexture2d, Texture2d, DepthFormat, UncompressedFloatFormat, MipmapsOption};
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);
        for x in &self.draw_areas{
            let depthtexture = DepthTexture2d::empty_with_format(&self.display, DepthFormat::F32, MipmapsOption::NoMipmap, x.res.0, x.res.1).unwrap();
            let area_tex = Texture2d::empty_with_format(&self.display, UncompressedFloatFormat::F32F32F32F32, MipmapsOption::NoMipmap, x.res.0, x.res.1).unwrap();

            let mut picking_target = SimpleFrameBuffer::with_depth_buffer(&self.display, &area_tex, &depthtexture).unwrap();

            picking_target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);
            let vertex_buf = self.box_vert_buf();
            let index_buffer = IndexBuffer::new(&self.display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
            for x in &self.draw_buffer.models{

            }
            let mat =
                [[x.size.0, 0.0, 0.0, 0.0],
                [0.0, x.size.1, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ x.pos.0 , x.pos.1, 0.0, 1.0f32]];

            if !x.distortion_shader {
                target.draw(
                    &vertex_buf,
                    &index_buffer,
                    self.shaders.get("solid_2d").unwrap(),
                    &uniform! { tex: &area_tex, matrix: mat},
                    &get_params()
                ).unwrap();
            }
            else{

            }
        }
        target.finish().unwrap();
    }
    pub fn add_shader(&mut self, name: &'static str, vert: &'static str, frag: &'static str){
        let program = Program::from_source(&self.display, vert, frag, None).unwrap();
        self.shaders.insert(name, program);
    }
    pub fn add_draw_area(&mut self, res: (u32, u32), size: (f32, f32), pos: (f32, f32), distortion_shader: bool){
        let draw_area = DrawArea{
            res,
            size,
            pos,
            distortion_shader
        };
        self.draw_areas.push(draw_area);
    }
    fn box_vert_buf(&self) -> VertexBuffer<Vertex2D>{
        let vert_buf = VertexBuffer::new(&self.display,
            &[
                Vertex2D { position: [ 0.0, 0.0 ]},
                Vertex2D { position: [ 1.0, 0.0 ]},
                Vertex2D { position: [ 1.0,  1.0 ]},
                Vertex2D { position: [ 0.0,  1.0 ]},
            ]
        ).unwrap();
        vert_buf
    }
}

pub fn get_params() -> DrawParameters<'static>{
    use glium::{Depth, draw_parameters, DepthTest};
    DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: draw_parameters::Blend::alpha_blending(),
        .. Default::default()
    }
}
