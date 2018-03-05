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
    use std::thread;
    use std::sync::mpsc::channel;
    use std::sync::mpsc;

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
    let posx = rand::thread_rng().gen_range(-10.0, 10.0) ;
    let posy = rand::thread_rng().gen_range(-10.0, 10.0);
    let posz = 0.0;
    //Create communication channels
    let (tx_player, rx_player) = channel::<player::Player>();
    let (tx_orient, rx_orient) = channel::<(f32,f32,f32)>();
    {
        let posx = posx.clone();
        let posy = posy.clone();
        let posz = posz.clone();
        thread::spawn(move || {
            let mut client = network::Network::new();
            println!("Connecting to server...");
            //Conecting to server
            client.connect("127.0.0.1:4587");
            println!("Done!");
            //Spawning player.
            let mut player = player::Player{
                id: 0,
                position: (posx, posy, posz),
                rotation: (0.0, 0.0, 0.0),
                model: "./assets/monkey.obj".to_string(),
                name: "None".to_string()
            };
            client.send(player.to_network(), 2);
            loop{
                let data = rx_orient.try_iter().last();
                if !data.is_none() {
                    let data = data.unwrap();
                    player.rotation = data;
                }
                client.check(&tx_player);
                client.send(player.to_network(), 2);
            }
        });
    }
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
    camera.set_pos(nalgebra::Vector3::new(posx,posy,posz));
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

    println!("Done!");
    //Starting main loop
    loop{
        let sys_time = SystemTime::now();


        let data = rx_player.try_iter().last();
        if !data.is_none() {
            let data = data.unwrap();
            let (x,y,z) = data.rotation;
            let mut new_player = render::RenderObject{
                mesh_name: "./assets/models/monkey.obj".to_string(),
                tex_name: "none".to_string(),
                position: data.position,
                rotation: (x,y,z + 3.14)
            };
            render_data.render_obj_buf.insert(data.id, new_player);
            playerlist.insert(data.id, data);
        }
        ohmd_context.update();
        let ohmd_orient = ohmd_device.getf(openhmd_rs::ohmd_float_value::OHMD_ROTATION_QUAT);
        let (x,y,z) = UnitQuaternion::from_quaternion(Quaternion::new(-ohmd_orient[0], ohmd_orient[3], ohmd_orient[2], ohmd_orient[1])).to_euler_angles();

        tx_orient.send((x,y,z));
        //Render
        display.draw(&render_data, &program, &ohmd_device, &camera, (scrw, scrh));


        let elapsed = sys_time.elapsed().unwrap();
        let fps = 1000 / ((elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64 + 1);
        //println!("FPS: {}", fps);
    }
}
