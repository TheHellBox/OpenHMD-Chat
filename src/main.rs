#[macro_use]
pub extern crate glium;
#[macro_use]
pub extern crate bytevec;

pub extern crate image;
pub extern crate openhmd_rs;
pub extern crate nalgebra;
pub extern crate cobalt;
pub extern crate rand;

mod render;
mod support;
mod network;
mod player;

use render::window::Window;

fn main(){
    //Some include stuff
    use std::env;
    use nalgebra::geometry::UnitQuaternion;
    use nalgebra::geometry::Quaternion;
    use rand::Rng;
    use std::collections::HashMap;
    use std::time::SystemTime;
    use std::thread::Thread;
    use std::sync::mpsc;
    use std::ffi::CStr;

    println!("Hello world!");
    //Some other stuff...
    let args: Vec<_> = env::args().collect();
    let mut scrw: u32 = 1024;
    let mut scrh: u32 = 768;
    let hmdid = {
        if args.len() > 1 {
            1
        }
        else{
            0
        }
    };
    //Create communication channel
    let (tx, rx): (mpsc::Sender<u32>, mpsc::Receiver<u32>) = mpsc::channel();
    //Init OpenHMD
    println!("VR mode");
    let ohmd_context = openhmd_rs::Context::new();
    ohmd_context.probe();
    println!("Opening device 0...");
    let ohmd_device = ohmd_context.list_open_device(hmdid);

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

    let mut playerlist: HashMap<u32, player::Player>  = HashMap::with_capacity(128);

    //Network...
    let mut client = network::Network::new();


    println!("Opening window...");
    let mut window = Window::new(scrw,scrh,"test");

    let (display, events_loop) = {
        window.get_display()
    };

    println!("Done!");
    //Loading some assets
    let mesh_buffer = support::obj_model_loader::gen_buffer(&display.display);
    let mut render_obj_buffer: HashMap<u32, render::RenderObject> = HashMap::with_capacity(1024);

    let mut camera = render::camera::Camera::new();

    let mut eyes = render::camera::Eyes::new();
    //Creating buffers and other stuff
    let mut render_data = render::RenderData{
        mesh_buf: mesh_buffer,
        render_obj_buf: render_obj_buffer,
    };
    let test_object = render::RenderObject{
        mesh_name: "./assets/models/scene.obj".to_string(),
        tex_name: "none".to_string(),
        position: (0.0,0.0,0.0),
        rotation: (0.0, 0.0, 0.0)
    };
    render_data.render_obj_buf.insert(1, test_object);
    println!("Building program...");

    let program = glium::Program::from_source(&display.display, &render::shader_distortion_vert, &render::shader_distortion_frag, None).unwrap();
    //Spawning player. FIXME: Apply position to local
    let posx = rand::thread_rng().gen_range(-0.1, 0.1) ;
    let posy = rand::thread_rng().gen_range(-0.1, 0.1);
    let posz = 0.0;

    let mut player = player::Player{
        id: 0,
        position: (posx, posy, posz),
        rotation: (0.0, 0.0, 0.0),
        model: "./assets/monkey.obj".to_string(),
        name: "None".to_string()
    };

    println!("Done!");

    println!("Connecting to server...");
    //Conecting to server
    client.connect("127.0.0.1:4587");
    client.check(&mut playerlist, &mut render_data);
    println!("Done!");
    //Starting main loop
    loop{
        let sys_time = SystemTime::now();
        ohmd_context.update();
        let ohmd_orient = ohmd_device.getf(openhmd_rs::ohmd_float_value::OHMD_ROTATION_QUAT);
        let (x,y,z) = UnitQuaternion::from_quaternion(Quaternion::new(-ohmd_orient[0], ohmd_orient[3], ohmd_orient[2], ohmd_orient[1])).to_euler_angles();
        player.rotation = (x,y,z);
        //Network
        //FIXME: Need multithread support for network
        client.send(player.to_network(), 2);
        println!("{:?}", &playerlist);
        client.check(&mut playerlist, &mut render_data);
        //Render
        display.draw(&render_data, &program, &eyes, &ohmd_device, (scrw, scrh));
        let elapsed = sys_time.elapsed().unwrap();
        println!("Elapsed: {} ms",
          (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    }
}
