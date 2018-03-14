use glium::{Display, Program};
use render;
use render::OhmdVertex;
use glium;
use openhmd_rs;
use nalgebra;

pub struct Draw_Display{
    pub display: Display
}

impl Draw_Display{
    pub fn draw(&self, buf: &render::RenderData, prog: &Program, ohmd_prog: &Program, device: &openhmd_rs::Device,camera: &render::camera::Camera,
                                                                scr: (u32,u32), mode: &render::window::RenderMode, hmd_params: &render::HMDParams){
        use glium::Surface;
        use nalgebra::geometry::{UnitQuaternion, Quaternion};
        use nalgebra::core::Vector4;
        let mut target = self.display.draw();

        let (scrw, scrh) = scr;
        let scrsize = match mode{
            &render::window::RenderMode::VR => scrw / 2,
            &render::window::RenderMode::Desktop => scrw,
        };
        target.clear_color_and_depth((0.2, 0.2, 0.7, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            viewport: Some(glium::Rect{left: 0, bottom: 0, width: scrsize, height: scrh}),
            .. Default::default()
        };

        let params_eye2 = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            viewport: Some(glium::Rect{left: scrsize, bottom: 0, width: scrw / 2, height: scrh}),
            .. Default::default()
        };
        let params_dis = glium::DrawParameters {
            .. Default::default()
        };
        let mut view = camera.view.to_homogeneous();
        let omodelv1 = mat_to_nalg(m16_to_4x4(device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX)));
        let omodelv1 = nalg_to_4x4((omodelv1* view));

        let omodelv2 = mat_to_nalg(m16_to_4x4(device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX)));
        let omodelv2 = nalg_to_4x4((omodelv2 * view));

        let oproj = m16_to_4x4(device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_PROJECTION_MATRIX));
        let oproj2 = m16_to_4x4(device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX));

        for (id, object) in &buf.render_obj_buf {
            if object.visible == true{
                let (rotx, roty, rotz, rotw) = object.rotation;
                let (sizex, sizey, sizez) = object.size;
                let (x, y, z) = object.position;
                let rotmatrix = UnitQuaternion::from_quaternion(Quaternion::new(rotx, roty, rotz, rotw)).to_homogeneous();
                //println!("{:?}", rotmatrix);

                let matrix = nalg_to_4x4(mat_to_nalg([
                    [sizex, 0.0, 0.0, 0.0],
                    [0.0, sizey, 0.0, 0.0],
                    [0.0, 0.0, sizez, 0.0],
                    [ x , y, z, 1.0f32],
                ]) * rotmatrix);

                let mesh = match buf.mesh_buf.get(&object.mesh_name) {
                    Some(x) => x,
                    None => { buf.mesh_buf.get("./assets/models/monkey.obj").unwrap() },
                    _ => { buf.mesh_buf.get("./assets/models/monkey.obj").unwrap() }
                };
                let tex = match buf.texture_buf.get(&object.tex_name) {
                    Some(x) => x,
                    None => { buf.texture_buf.get("./assets/textures/test.png").unwrap() },
                    _ => { buf.texture_buf.get("./assets/textures/test.png").unwrap() }
                };
                //println!("{}", &object.mesh_name);
                //LensCenter: lens_center,ViewportScale: viewport_scale, WarpScale: warp_scale, HmdWarpParam: hmd_params.distortion_k, aberr: hmd_params.aberration_k
                target.draw(
                    &mesh.mesh,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    prog,
                    &uniform! { matrix: matrix, perspective: oproj, view: omodelv1, tex: tex},
                    &params
                ).unwrap();
                if mode == &render::window::RenderMode::VR {
                    target.draw(
                        &mesh.mesh,
                        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                        prog,
                        &uniform! { matrix: matrix, perspective: oproj2, view: omodelv2, tex: tex},
                        &params_eye2
                    ).unwrap();
                }
            }
        }



        let warp_tex = buf.texture_buf.get("./assets/textures/lookmumigotsomething.png").unwrap();
        let warp_scale = hmd_params.left_lens_center[0] / hmd_params.right_lens_center[0];

        let vert_buf = glium::VertexBuffer::new(&self.display,
            &[
                OhmdVertex { coords: [ 0.0, 0.0 ]},
                OhmdVertex { coords: [ 1.0, 0.0 ]},
                OhmdVertex { coords: [ 1.0,  2.0 ]},
                OhmdVertex { coords: [ 0.0,  2.0 ]},
            ]
        ).unwrap();

        let index_buffer = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3]).unwrap();

        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ 0.0 , -1.0, 0.0, 1.0f32],
        ];

        let matrix2 = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ -1.0 , -1.0, 0.0, 1.0f32],
        ];

        target.draw(
            &vert_buf,
            &index_buffer,
            &ohmd_prog,
            &uniform! { warpTexture: warp_tex, mvp: matrix, LensCenter: hmd_params.left_lens_center,ViewportScale: hmd_params.view_port_scale, WarpScale: warp_scale,
                HmdWarpParam: hmd_params.distortion_k, aberr: hmd_params.aberration_k},
            &params_dis
        ).unwrap();

        target.draw(
            &vert_buf,
            &index_buffer,
            &ohmd_prog,
            &uniform! { warpTexture: warp_tex, mvp: matrix2, LensCenter: hmd_params.right_lens_center,ViewportScale: hmd_params.view_port_scale, WarpScale: warp_scale,
                HmdWarpParam: hmd_params.distortion_k, aberr: hmd_params.aberration_k},
            &params_dis
        ).unwrap();

        target.finish().unwrap();
    }
}

fn m16_to_4x4(mat: [f32; 16]) -> [[f32;4]; 4]{
    let mat = [
        [mat[0], mat[1], mat[2], mat[3]],
        [mat[4], mat[5], mat[6], mat[7]],
        [mat[8], mat[9], mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ];
    mat
}

fn nalg_to_4x4(mat: nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4>) -> [[f32;4]; 4]{
    let mat = [
        [mat[0], mat[1], mat[2], mat[3]],
        [mat[4], mat[5], mat[6], mat[7]],
        [mat[8], mat[9], mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ];
    mat
}

fn mat_to_nalg(mat: [[f32;4]; 4]) -> nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4>{
    let mut raw: nalgebra::core::MatrixN<f32, nalgebra::core::dimension::U4> = nalgebra::core::MatrixN::new_scaling(0.0);
    for x in 0..4{
        for y in 0..4{
            raw[y*4 + x] = mat[y][x];
        }
    }
    raw
}
fn add_matrix(mat1: [[f32;4]; 4], mat2: [[f32;4]; 4])  -> [[f32;4]; 4]{
    let mat = [
        [mat1[0][0] + mat2[0][0], mat1[0][1] + mat2[0][1], mat1[0][2] + mat2[0][2], mat1[0][3] + mat2[0][3]],
        [mat1[1][0] + mat2[1][0], mat1[1][1] + mat2[1][1], mat1[1][2] + mat2[1][2], mat1[1][3] + mat2[1][3]],
        [mat1[2][0] + mat2[2][0], mat1[2][1] + mat2[2][1], mat1[2][2] + mat2[2][2], mat1[2][3] + mat2[2][3]],
        [mat1[3][0] + mat2[3][0], mat1[3][1] + mat2[3][1], mat1[3][2] + mat2[3][2], mat1[3][3] + mat2[3][3]],
    ];
    mat
}
