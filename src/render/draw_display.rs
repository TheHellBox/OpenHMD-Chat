use glium::{Display, Program};
use render;
use render::OhmdVertex;
use glium;
use openhmd_rs;
use nalgebra;
use openhmd;
use math::*;

pub struct DrawDisplay{
    pub display: Display
}

impl DrawDisplay{
    pub fn draw(&self, buf: &render::RenderData, prog: &Program, ohmd_prog: &Program, device: &openhmd_rs::Device,camera: &render::camera::Camera,
                                                                scr: (u32,u32), mode: &render::window::RenderMode, hmd_params: &openhmd::HMDParams){
        use glium::Surface;
        use nalgebra::geometry::{UnitQuaternion, Quaternion};
        let mut target = self.display.draw();

        let (scrw, scrh) = scr;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let params_dis = glium::DrawParameters {
            .. Default::default()
        };

        let mut picking_attachments: Option<(glium::texture::UnsignedTexture2d, glium::framebuffer::DepthRenderBuffer)> = None;

        let picking_pbo: glium::texture::pixel_buffer::PixelBuffer<u32>
            = glium::texture::pixel_buffer::PixelBuffer::new_empty(&self.display, 6220800);

        if picking_attachments.is_none() || (
            picking_attachments.as_ref().unwrap().0.get_width(),
            picking_attachments.as_ref().unwrap().0.get_height().unwrap()
        ) != target.get_dimensions() {
            let (width, height) = target.get_dimensions();
            picking_attachments = Some((
                glium::texture::UnsignedTexture2d::empty_with_format(
                    &self.display,
                    glium::texture::UncompressedUintFormat::U32,
                    glium::texture::MipmapsOption::NoMipmap,
                    width, height,
                ).unwrap(),
                glium::framebuffer::DepthRenderBuffer::new(
                    &self.display,
                    glium::texture::DepthFormat::F32,
                    width, height,
                ).unwrap()
            ))
        }

        target.clear_color_and_depth((0.2, 0.2, 0.7, 1.0), 1.0);

        let depthtexture1 = glium::texture::DepthTexture2d::empty_with_format(&self.display, glium::texture::DepthFormat::F32, glium::texture::MipmapsOption::NoMipmap, scrw, scrh).unwrap();
        let eye1_tex = glium::texture::Texture2d::empty_with_format(&self.display, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, scrw, scrh).unwrap();

        let depthtexture2 = glium::texture::DepthTexture2d::empty_with_format(&self.display, glium::texture::DepthFormat::F32, glium::texture::MipmapsOption::NoMipmap, scrw, scrh).unwrap();
        let eye2_tex = glium::texture::Texture2d::empty_with_format(&self.display, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, scrw, scrh).unwrap();

        let mut picking_target1 = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&self.display, &eye1_tex, &depthtexture1).unwrap();
        let mut picking_target2 = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&self.display, &eye2_tex, &depthtexture2).unwrap();
        picking_target1.clear_color_and_depth((0.2, 0.2, 0.7, 1.0), 1.0);
        picking_target2.clear_color_and_depth((0.2, 0.2, 0.7, 1.0), 1.0);


        let view = camera.view.to_homogeneous();

        let omodelv1 = nalg_to_4x4(mat16_to_nalg(device.getf(openhmd_rs::ohmd_float_value::OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX)) * view);

        let omodelv2 = nalg_to_4x4(mat16_to_nalg(device.getf(openhmd_rs::ohmd_float_value::OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX)) * view);


        for (_, object) in &buf.render_obj_buf {
            if object.visible == true{
                let (rotx, roty, rotz, rotw) = object.rotation;
                let (scalex, scaley, scalez) = object.scale;
                let (x, y, z) = object.position;
                let rotmatrix = UnitQuaternion::from_quaternion(Quaternion::new(rotx, roty, rotz, rotw)).to_homogeneous();
                //println!("{:?}", rotmatrix);

                let matrix = nalg_to_4x4(mat_to_nalg([
                    [scalex, 0.0, 0.0, 0.0],
                    [0.0, scaley, 0.0, 0.0],
                    [0.0, 0.0, scalez, 0.0],
                    [ x , y, z, 1.0f32],
                ]) * rotmatrix);

                let mesh = match buf.mesh_buf.get(&object.mesh_name) {
                    Some(x) => x,
                    None => { buf.mesh_buf.get("./assets/models/cube.obj").unwrap() },
                    _ => { buf.mesh_buf.get("./assets/models/cube.obj").unwrap() }
                };
                let tex = match buf.texture_buf.get(&object.tex_name) {
                    Some(x) => x,
                    None => { buf.texture_buf.get("./assets/textures/test.png").unwrap() },
                    _ => { buf.texture_buf.get("./assets/textures/test.png").unwrap() }
                };
                //println!("{}", &object.mesh_name);
                //LensCenter: lens_center,ViewportScale: viewport_scale, WarpScale: warp_scale, HmdWarpParam: hmd_params.distortion_k, aberr: hmd_params.aberration_k
                //let mut picking_target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, picking_texture, depth_buffer).unwrap();

                picking_target1.draw(
                    &mesh.mesh,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    prog,
                    &uniform! { matrix: matrix, perspective: hmd_params.projection1, view: omodelv1, tex: tex},
                    &params
                ).unwrap();
                //if mode == &render::window::RenderMode::VR {
                    picking_target2.draw(
                        &mesh.mesh,
                        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                        prog,
                        &uniform! { matrix: matrix, perspective: hmd_params.projection2, view: omodelv2, tex: tex},
                        &params
                    ).unwrap();
                //}
            }
        }

        let warp_scale = hmd_params.left_lens_center[0] / hmd_params.right_lens_center[0];

        let vert_buf = glium::VertexBuffer::new(&self.display,
            &[
                OhmdVertex { coords: [ 0.0, 0.0 ]},
                OhmdVertex { coords: [ 1.0, 0.0 ]},
                OhmdVertex { coords: [ 1.0,  1.0 ]},
                OhmdVertex { coords: [ 0.0,  1.0 ]},
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
            &uniform! {  warpTexture: &eye1_tex, mvp: matrix, LensCenter: hmd_params.left_lens_center,ViewportScale: hmd_params.view_port_scale, WarpScale: warp_scale,
                HmdWarpParam: hmd_params.distortion_k, aberr: hmd_params.aberration_k},
            &params_dis
        ).unwrap();

        target.draw(
            &vert_buf,
            &index_buffer,
            &ohmd_prog,
            &uniform! {  warpTexture: &eye2_tex, mvp: matrix2, LensCenter: hmd_params.right_lens_center,ViewportScale: hmd_params.view_port_scale, WarpScale: warp_scale,
                HmdWarpParam: hmd_params.distortion_k, aberr: hmd_params.aberration_k},
            &params_dis
        ).unwrap();

        target.set_finish().unwrap();
    }
}
