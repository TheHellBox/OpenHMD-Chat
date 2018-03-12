#[macro_use]
pub extern crate glium;
#[macro_use]
pub extern crate bytevec;

pub extern crate image;
pub extern crate openhmd_rs;
pub extern crate nalgebra;
pub extern crate cobalt;
pub extern crate rand;
pub extern crate gilrs;
pub extern crate alto;
pub extern crate opus;

mod render;
mod support;
mod network;
mod player;
mod audio;
mod vr_gui;

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
    use gilrs::{Gilrs, Button, Event, EventType};
    use audio::AudioMsg;
    use render::window::RenderMode;

    //Some other stuff...
    let args: Vec<_> = env::args().collect();
    let mut scrw: u32 = 1024;
    let mut scrh: u32 = 768;
    let ip = {
        if args.len() > 1 {
            args[1].clone()
        }
        else{
            "127.0.0.1:4587".to_string()
        }
    };
    let mut vrmode = RenderMode::VR;
    let hmdid = {
        if args.len() > 2 {
            vrmode = RenderMode::Desktop;
            1
        }
        else{
            vrmode = RenderMode::VR;
            0
        }
    };
    //Generating random player position
    let mut posx = rand::thread_rng().gen_range(-7.0, 7.0);
    let mut posy = 0.0;
    let mut posz = rand::thread_rng().gen_range(-7.0, 7.0);
    //Create communication channels
    let (tx_player, rx_player) = channel::<player::Player>();
    let (tx_players, rx_players) = channel::<HashMap<u32, player::Player>>();
    let (tx_orient, rx_orient) = channel::<((f32,f32,f32,f32), (f32,f32,f32))>();
    let (tx_netsound_in, rx_netsound_in) = channel::<AudioMsg>();
    let (tx_netsound_out, rx_netsound_out) = channel::<AudioMsg>();
    {
        thread::spawn(move || {
            let mut client = network::Network::new();
            println!("Connecting to server...");
            //Conecting to server
            client.connect(ip);
            println!("Connected!");
            //Spawning player.
            let mut player = player::Player{
                id: 0,
                position: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0, 0.0),
                model: "./assets/monkey.obj".to_string(),
                name: "None".to_string()
            };
            client.send(player.to_network(), 2);
            loop{
                let data = rx_orient.try_iter();
                for data in data{
                    let (rot, pos) = data;
                    player.rotation = rot;
                    player.position = pos;
                }
                let netsound = rx_netsound_in.try_iter();
                for x in netsound{
                    let netsound = x;
                    let data = network::NetAudio{
                        data: netsound.data,
                        id: player.id
                    };
                    client.send(data.to_network(), 3);
                }
                client.check(&tx_player, &tx_netsound_out, &player);
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

    println!("\nDevice description: ");
    println!("Vendor:   {}", ohmd_context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_VENDOR));
    println!("Product:  {}", ohmd_context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_PRODUCT));
    println!("Path:     {}\n", ohmd_context.list_gets(0, openhmd_rs::ohmd_string_value::OHMD_PATH));

    scrw = ohmd_device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION) as u32;
    scrh = ohmd_device.geti(openhmd_rs::ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION) as u32;

    println!("HMD scrw res {}", scrw);
    println!("HMD scrh res {}", scrh);

    let mut playerlist: HashMap<u32, player::Player>  = HashMap::with_capacity(128);

    println!("Opening window...");
    let mut window = Window::new(scrw,scrh, "test", &vrmode);

    let (display, events_loop) = window.get_display();

    println!("Done!");
    //Loading some assets
    println!("");
    let mesh_buffer = support::obj_model_loader::gen_buffer(&display.display);
    println!("");
    let texture_buffer = support::texture_loader::gen_texture_buffer(&display.display);
    println!("");
    let mut render_obj_buffer: HashMap<u32, render::RenderObject> = HashMap::with_capacity(1024);

    let mut camera = render::camera::Camera::new();
    camera.set_pos(nalgebra::Vector3::new(posx,posy,posz));
    //Creating buffers and other stuff
    let mut render_data = render::RenderData{
        mesh_buf: mesh_buffer,
        texture_buf: texture_buffer,
        render_obj_buf: render_obj_buffer,
    };
    let scene = render::RenderObject{
        mesh_name: "./assets/models/scene.obj".to_string(),
        tex_name: "./assets/textures/background.png".to_string(),
        position: (0.0,0.0,0.0),
        rotation: (0.0, 0.0, 1.0, 0.0),
        size: (1.0, 1.0, 1.0),
        visible: true
    };
    render_data.render_obj_buf.insert(1, scene);

    println!("Building program...");
    let program = glium::Program::from_source(&display.display, &render::shader_distortion_vert, &render::shader_distortion_frag, None).unwrap();
    println!("Done!");
    //gamepad stuff
    let mut gilrs = Gilrs::new().unwrap();
    //Glium Text
    //let gli_text = glium_text::TextSystem::new(&display);
    //let font = glium_text::FontTexture::new(&display, std::fs::File::open(&std::path::Path::new("./assets/fonts/Roboto-Regular.ttf")).unwrap(), 24).unwrap();
    //let text = glium_text::TextDisplay::new(&system, &font, "Hello world!");

    let mut vr_gui = vr_gui::VRGui::new();
    let test_element = vr_gui::Element::new((3.0,0.0), (1.0,2.0), (false, "./assets/textures/test.png".to_string()), vr_gui::ElementType::Panel);
    let test_element2 = vr_gui::Element::new((0.0,0.0), (1.0,1.0), (false, "./assets/textures/test.png".to_string()), vr_gui::ElementType::Panel);
    vr_gui.push_element(test_element);
    vr_gui.push_element(test_element2);

    let mut settings_active = false;

    let mut player_speed_forward = 0.0;
    let mut player_speed_lr = 0.0;
    let mut player_moving = false;

    let mut posx_ghost = -posx;
    let mut posz_ghost = -posy;
    // Audio stuff
    thread::spawn(move || {
        audio::start_audio(&tx_netsound_in, &rx_netsound_out, &rx_players);
    });
    //Starting main loop
    loop{
        let sys_time = SystemTime::now();
        let data = rx_player.try_iter();
        for data in data{
            let (x,y,z,w) = data.rotation;
            let mut new_player = render::RenderObject{
                mesh_name: "./assets/models/monkey.obj".to_string(),
                tex_name: "./assets/textures/test.png".to_string(),
                position: data.position,
                rotation: (x,y,z,w),
                size: (1.0, 1.0, 1.0),
                visible: true
            };
            render_data.render_obj_buf.insert(data.id, new_player);
            playerlist.insert(data.id, data);
        }
        ohmd_context.update();

        if args.len() == 2 {
            while let Some(event) = gilrs.next_event() {
                match event {
                    Event { id, event: EventType::AxisChanged(gilrs::ev::Axis::LeftStickY, val1, val2), .. } => {
                        if val1 > 0.1{
                            player_speed_forward = 0.2 * val1;
                        }
                        else if val1 < -0.1{
                            player_speed_forward = 0.2 * val1;
                        }
                        else if (val1 > -0.1) & (val1 < 0.1){
                            player_speed_forward = 0.0;
                        }

                    }
                    Event { id, event: EventType::AxisChanged(gilrs::ev::Axis::LeftStickX, val1, val2), .. } => {
                        if val1 > 0.1{
                            player_speed_lr = 0.2 * val1;
                        }
                        else if val1 < -0.1{
                            player_speed_lr = 0.2 * val1;
                        }
                        else if (val1 > -0.1) & (val1 < 0.1){
                            player_speed_lr = 0.0;
                        }
                    }
                    Event { id, event: EventType::ButtonPressed(gilrs::ev::Button::Start, val1), .. } => {
                        settings_active = match settings_active{
                            false => true,
                            true => false,
                        };
                    }
                    _ => (),
                };
            }
        }

        if (player_speed_forward == 0.0) & (player_speed_lr == 0.0){
            player_moving = false;
            posx = -posx_ghost;
            posz = -posz_ghost;
        }
        else{
            player_moving = true;
        }
        for (id, x) in &vr_gui.elements{
            if x.el_type == vr_gui::ElementType::Panel {
                let (gui_posx, gui_posy) = x.position;
                let (gui_sizex, gui_sizey) = x.size;
                let (prop, texture) = x.container.clone();
                let object = render::RenderObject{
                    mesh_name: "./assets/models/cube.obj".to_string(),
                    tex_name: texture,
                    position: (-posx + gui_posx,-posy + gui_posy,-posz - 2.0),
                    rotation: (0.0, 0.0, 1.0, 0.0),
                    size: (gui_sizex, gui_sizey, 0.1),
                    visible: settings_active
                };
                render_data.render_obj_buf.insert(x.rend_id, object);
            }
        }
        let ohmd_orient = ohmd_device.getf(openhmd_rs::ohmd_float_value::OHMD_ROTATION_QUAT);
        camera.set_pos(nalgebra::Vector3::new(posx,posy,posz));
        //ohmd_device.setf(openhmd_rs::ohmd_float_value::OHMD_POSITION_VECTOR, ohmd_pos)
        let quat = UnitQuaternion::from_quaternion(Quaternion::new(ohmd_orient[0], -ohmd_orient[1], ohmd_orient[2], -ohmd_orient[3]));

        let matrix = quat.to_homogeneous();
        //Moving player
        if player_moving {
            posx_ghost -= matrix[8] * player_speed_forward + matrix[0] * player_speed_lr;
            posz_ghost -= matrix[10] * player_speed_forward + matrix[2] * player_speed_lr;
            let mut ghost = render::RenderObject{
                mesh_name: "./assets/models/monkey.obj".to_string(),
                tex_name: "./assets/textures/test.png".to_string(),
                position: (posx_ghost, posy, posz_ghost),
                rotation: (quat[0], quat[1], quat[2], quat[3]),
                size: (1.0, 1.0, 1.0),
                visible: true
            };
            render_data.render_obj_buf.insert(11119, ghost);
        }
        else{
            render_data.render_obj_buf.remove(&(11119 as u32) );
        }

        tx_orient.send(((quat[0],quat[1],quat[2],quat[3]), (posx,posy,posz)));
        //Render
        display.draw(&render_data, &program, &ohmd_device, &camera, (scrw, scrh), &vrmode);

        let elapsed = sys_time.elapsed().unwrap();
        let fps = 1000.0 / (((elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64) as f32 + 0.01);
        tx_players.send(playerlist.clone());
        //println!("FPS: {}", fps as u32);
    }
}
