use glium::{glutin, Display, Program};
use render;
use glium;

pub struct Draw_Display{
    pub display: Display
}

impl Draw_Display{
    pub fn draw(&self, buf: &render::RenderData, prog: &Program, cam: &render::camera::Camera){
        use glium::Surface;
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let matrix = [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [ 0.0 , 0.0, 0.0, 1.0f32],
        ];

        let proj = cam.persp.to_homogeneous().as_ref().to_owned();
        let view = cam.view.to_homogeneous().as_ref().to_owned();

        for (path, mesh) in &buf.mesh_buf {
            target.draw(
                &mesh.mesh,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                prog,
                &uniform! { matrix: matrix, perspective: proj, view: view },
                &params
            ).unwrap();
        }
        target.finish().unwrap();
    }
}
