use nalgebra::geometry::{Point3, UnitQuaternion, Translation3};
use glium::{ Surface, VertexBuffer, IndexBuffer, Program, DrawParameters};
use render::{Vertex2D, DrawArea, Window};
use glium::index::PrimitiveType;
use std::collections::HashMap;
use nalgebra::core::{Matrix4};
use glium::index::NoIndices;
use render::camera::Camera;
use render::model::Model;
use game::Game;

pub struct DrawObject{
    pub model: Model,
    pub position: Point3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: (f32, f32, f32),
    pub shader: &'static str
}

pub struct DrawBuffer{
    pub objects: HashMap<String, DrawObject>
}

impl Window{
    pub fn draw(&mut self, game: &Game){
        use glium::framebuffer::SimpleFrameBuffer;
        use glium::texture::{DepthTexture2d, Texture2d, DepthFormat, UncompressedFloatFormat, MipmapsOption};
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);
        for (_, x) in &self.draw_areas{
            let depthtexture = DepthTexture2d::empty_with_format(&self.display, DepthFormat::F32, MipmapsOption::NoMipmap, x.res.0, x.res.1).unwrap();
            let area_tex = Texture2d::empty_with_format(&self.display, UncompressedFloatFormat::F32F32F32F32, MipmapsOption::NoMipmap, x.res.0, x.res.1).unwrap();

            let mut picking_target = SimpleFrameBuffer::with_depth_buffer(&self.display, &area_tex, &depthtexture).unwrap();

            picking_target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);
            let vertex_buf = self.box_vert_buf();
            let index_buffer = IndexBuffer::new(&self.display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();

            let perspective: [[f32; 4]; 4] = x.camera.perspective.into();
            let view: [[f32; 4]; 4] = x.camera.view.into();

            for (name, game_object) in &game.gameobjects{
                if game_object.render_object != "none".to_string(){
                    if let Some(render_object) = self.draw_buffer.objects.get_mut(&game_object.render_object){
                        for mesh in &render_object.model.meshes{
                            let renderobj_transform = render_object.calc_transform();
                            let gameobj_transform = game_object.calc_transform();
                            let transform: [[f32; 4]; 4] = gameobj_transform.into();
                            picking_target.draw(
                                &mesh.mesh,
                                &NoIndices(PrimitiveType::TrianglesList),
                                self.shaders.get(render_object.shader).unwrap(),
                                &uniform! { matrix: transform, perspective: perspective, view: view, tex: &mesh.texture},
                                &get_params()
                            ).unwrap();
                        }
                    }
                }
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
    pub fn add_draw_area(&mut self, name: String, camera: Camera, res: (u32, u32), size: (f32, f32), pos: (f32, f32), distortion_shader: bool){
        let draw_area = DrawArea{
            camera,
            res,
            size,
            pos,
            distortion_shader
        };
        self.draw_areas.insert(name, draw_area);
    }
    pub fn add_draw_object(&mut self, name: String, model: Model, position: Point3<f32>, rotation: UnitQuaternion<f32>, scale: (f32, f32, f32), shader: &'static str){
        self.draw_buffer.objects.insert(
            name,
            DrawObject{
                model,
                position,
                rotation,
                scale,
                shader
            }
        );
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

impl DrawObject{
    pub fn calc_transform(&self) -> Matrix4<f32>{
        let scale_matrix: Matrix4<f32> = Matrix4::new(
            self.scale.0 as f32, 0.0, 0.0, 0.0,
            0.0, self.scale.1 as f32, 0.0, 0.0,
            0.0, 0.0, self.scale.2 as f32, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let translation_matrix = Translation3::from_vector(self.position.coords).to_homogeneous();
        let rotation_matrix = self.rotation.to_homogeneous();
        translation_matrix * scale_matrix * rotation_matrix
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
