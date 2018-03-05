use glium::{Display, Program};
use render;
use glium;
use openhmd_rs;

pub struct Draw_Display{
    pub display: Display
}

impl Draw_Display{
    pub fn draw(&self, buf: &render::RenderData, prog: &Program, device: &openhmd_rs::Device,camera: &render::camera::Camera, scr: (u32,u32)){
        use glium::Surface;
        use nalgebra::geometry::UnitQuaternion;
        let mut target = self.display.draw();

        let (scrw, scrh) = scr;
        target.clear_color_and_depth((0.2, 0.2, 0.4, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            viewport: Some(glium::Rect{left: 0, bottom: 0, width: scrw / 2, height: scrh}),
            .. Default::default()
        };

        let params_eye2 = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            viewport: Some(glium::Rect{left: scrw / 2, bottom: 0, width: scrw / 2, height: scrh}),
            .. Default::default()
        };


        let omodelv1 = device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX);
        let omodelv1 = [
            [omodelv1[0], omodelv1[1], omodelv1[2], omodelv1[3]],
            [omodelv1[4], omodelv1[5], omodelv1[6], omodelv1[7]],
            [omodelv1[8], omodelv1[9], omodelv1[10], omodelv1[11]],
            [camera.view.vector[0], camera.view.vector[1], camera.view.vector[2], 1.0],
        ];
        let omodelv2 = device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX);

        let omodelv2 = [
            [omodelv2[0], omodelv2[1], omodelv2[2], omodelv2[3]],
            [omodelv2[4], omodelv2[5], omodelv2[6], omodelv2[7]],
            [omodelv2[8], omodelv2[9], omodelv2[10], omodelv2[11]],
            [camera.view.vector[0], camera.view.vector[1], camera.view.vector[2], 1.0],
        ];

        let oproj = device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_PROJECTION_MATRIX);
        let oproj = [
            [oproj[0], oproj[1], oproj[2], oproj[3]],
            [oproj[4], oproj[5], oproj[6], oproj[7]],
            [oproj[8], oproj[9], oproj[10], oproj[11]],
            [oproj[12], oproj[13], oproj[14], oproj[15]],
        ];
        let oproj2 = device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX);
        let oproj2 = [
            [oproj2[0], oproj2[1], oproj2[2], oproj2[3]],
            [oproj2[4], oproj2[5], oproj2[6], oproj2[7]],
            [oproj2[8], oproj2[9], oproj2[10], oproj2[11]],
            [oproj2[12], oproj2[13], oproj2[14], oproj2[15]],
        ];

        for (id, object) in &buf.render_obj_buf {
            let (rotx, roty, rotz) = object.rotation;
            let (x, y, z) = object.position;
            let rotmatrix = UnitQuaternion::from_euler_angles(rotx, roty, rotz).to_rotation_matrix().unwrap();
            //println!("{:?}", rotmatrix);
            let matrix = [
                [0.1 + rotmatrix[0], 0.0 + rotmatrix[1], 0.0 + rotmatrix[2], 0.0],
                [0.0 + rotmatrix[3], 0.1 + rotmatrix[4], 0.0 + rotmatrix[5], 0.0],
                [0.0 + rotmatrix[6], 0.0 + rotmatrix[7], 0.1 + rotmatrix[8], 0.0],
                [ x , y, z, 1.0f32],
            ];
            let mesh = buf.mesh_buf.get(&object.mesh_name).unwrap();
            //println!("{}", &object.mesh_name);
            target.draw(
                &mesh.mesh,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                prog,
                &uniform! { matrix: matrix, perspective: oproj, view: omodelv1 },
                &params
            ).unwrap();
            target.draw(
                &mesh.mesh,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                prog,
                &uniform! { matrix: matrix, perspective: oproj2, view: omodelv2,u_light: [-1.0, 0.4, 0.9f32]},
                &params_eye2
            ).unwrap();
        }
        target.finish().unwrap();
    }
}
