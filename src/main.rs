#[macro_use]
pub extern crate glium;
pub extern crate image;
pub extern crate openhmd_rs;
pub extern crate nalgebra;

mod render;
mod support;

use render::window::Window;

fn main(){
    use std::env;
    println!("Hello world!");

    let args: Vec<_> = env::args().collect();

    let mut scrw: u32 = 1024;
    let mut scrh: u32 = 768;

    //if args.len() > 1 {

        use std::ffi::CStr;
        println!("VR mode");
        let ohmd_context = openhmd_rs::Context::new();
        ohmd_context.probe();
        println!("Opening device 0...");
        let ohmd_device = ohmd_context.list_open_device(0);

        ohmd_context.update();

        println!("");

        println!("Device description: ");
        println!("Vendor:   {}", ohmd_context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_VENDOR));
        println!("Product:  {}", ohmd_context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_PRODUCT));
        println!("Path:     {}", ohmd_context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_PATH));

        println!("");

        scrw = ohmd_device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION) as u32;
        scrh = ohmd_device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION) as u32;

        println!("HMD scrw res {}", scrw);
        println!("HMD scrh res {}", scrh);

    //}
    //else{
    //    println!("Simple mode");
    //}
    println!("Opening window...");
    let mut window = Window::new(scrw,scrh,"test");

    let (display, events_loop) = {
        window.get_display()
    };

    println!("Done!");

    let mesh_buffer = support::obj_model_loader::gen_buffer(&display.display);
    let mut render_obj_buffer: Vec<render::RenderObject> = Vec::with_capacity(1024);

    let mut camera = render::camera::Camera::new();

    let render_data = render::RenderData{
        mesh_buf: mesh_buffer,
        render_obj_buf: render_obj_buffer,
    };

    println!("Building program...");

    let program = glium::Program::from_source(&display.display, &render::shader_distortion_vert, &render::shader_distortion_frag, None).unwrap();

    println!("Done!");

    loop{
        ohmd_context.update();
        let ohmd_orient = ohmd_device.getf(openhmd_rs::ohmd_float_value::OHMD_ROTATION_QUAT);
        println!("{:?}", ohmd_orient);
        camera.set_rot(nalgebra::geometry::UnitQuaternion::from_quaternion(nalgebra::geometry::Quaternion::new(ohmd_orient[0], ohmd_orient[3], -ohmd_orient[2], ohmd_orient[1])));
        display.draw(&render_data, &program, &camera);

    }
}
