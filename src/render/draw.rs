use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion, Translation3};
use glium::{ Surface, VertexBuffer, IndexBuffer, Program, DrawParameters};
use render::{Vertex2D, DrawArea, Window};
use glium::index::PrimitiveType;
use nalgebra::core::{Matrix4};
use glium::index::NoIndices;
use render::camera::Camera;
use render::model::Model;
pub struct Draw_Object{
    model: Model,
    position: Point3<f32>,
    rotation: UnitQuaternion<f32>,
    scale: (f32, f32, f32)
}

pub struct Draw_Buffer{
    pub objects: Vec<Draw_Object>
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

            let perspective: [[f32; 4]; 4] = x.camera.perspective.into();
            let view: [[f32; 4]; 4] = x.camera.view.into();

            for object in &self.draw_buffer.objects{
                for mesh in &object.model.meshes{
                    let transform: [[f32; 4]; 4] = object.calc_transform().into();
                    picking_target.draw(
                        &mesh.mesh,
                        &NoIndices(PrimitiveType::TrianglesList),
                        self.shaders.get("simple").unwrap(),
                        &uniform! { matrix: transform, perspective: perspective, view: view, tex: &mesh.texture},
                        &get_params()
                    ).unwrap();
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
    pub fn add_draw_area(&mut self, camera: Camera, res: (u32, u32), size: (f32, f32), pos: (f32, f32), distortion_shader: bool){
        let draw_area = DrawArea{
            camera,
            res,
            size,
            pos,
            distortion_shader
        };
        self.draw_areas.push(draw_area);
    }
    pub fn add_draw_object(&mut self, model: Model, position: Point3<f32>, rotation: UnitQuaternion<f32>, scale: (f32, f32, f32)){
        self.draw_buffer.objects.push(
            Draw_Object{
                model,
                position,
                rotation,
                scale
            }
        )
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

impl Draw_Object{
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
