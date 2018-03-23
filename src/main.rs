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
pub extern crate json;
pub extern crate ncollide;

mod render;
mod support;
mod network;
mod player;
mod audio;
mod vr_gui;
mod gameplay;
mod openhmd;
mod math;

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
    use std::sync::mpsc::{channel, sync_channel};
    use gilrs::{Gilrs};
    use audio::AudioMsg;
    use render::window::RenderMode;
    use std::io::prelude::*;
    use std::fs::File;
    //Some other stuff...
    let args: Vec<_> = env::args().collect();
    let mut scrw: u32 = 1024;
    let mut scrh: u32 = 768;
    let ip = {
        if args.len() > 1 {
            args[1].clone()
        }
        else{
            println!("Please, enter IP. Example: ./openhmdchat ip:port");
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
    //Create communication channels. FIXME: Move all this MPSC stuff away from main
    let (tx_player, rx_player) = channel::<player::Player>();
    let (tx_mapobj, rx_mapobj) = channel::<support::map_loader::MapObject>();
    let (tx_players, rx_players) = channel::<HashMap<u32, player::Player>>();
    let (tx_orient, rx_orient) = channel::<((f32,f32,f32,f32), (f32,f32,f32))>();
    let (tx_netsound_in, rx_netsound_in) = channel::<AudioMsg>();
    let (tx_netsound_out, rx_netsound_out) = channel::<AudioMsg>();
    let (tx_ready, rx_ready) = channel::<bool>();
    //FIXME: Move network thread from main to other module, it takes a lot of code
    {

        thread::spawn(move || {
            let params = network::client_params::ClParams::new();
            let mut client = network::Network::new();
            println!("Connecting to server...");
            //Conecting to server
            client.connect(ip);

            //Spawning player.
            let mut player = player::Player{
                id: 0,
                position: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0, 0.0),
                model: "./assets/monkey.obj".to_string(),
                name: "None".to_string()
            };

            client.check(&tx_player,&tx_mapobj, &tx_netsound_out, &player);
            println!("Sending client info...");
            let params = params.to_network();
            client.send(params, 4, cobalt::MessageKind::Reliable);
            let mut file_writer = support::file_writer::FileWriter::new("temp".to_string());
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
                    client.send(data.to_network(), 3, cobalt::MessageKind::Instant);
                }
                client.check(&tx_player,&tx_mapobj, &tx_netsound_out, &player);
                let back_data = client.rx_back.try_iter();
                for (x, type_d) in back_data{
                    if x.starts_with(&vec![233, 144, 122, 198, 134, 253, 251]){
                        file_writer = support::file_writer::FileWriter::new(String::from_utf8(x[7..x.len()].to_vec()).unwrap());
                        println!("Downloading {}", String::from_utf8(x[7..x.len()].to_vec()).unwrap());
                    }
                    else if x.starts_with(&vec![100, 137, 211, 233, 212, 222]){
                        tx_ready.send(true);
                    }
                    else{
                        file_writer.write((&x).to_owned());
                    }
                }
            }
        });
    }
    let data = rx_ready.recv();
    //Init OpenHMD
    let hmd = openhmd::ohmdHeadSet::new(hmdid);
    hmd.context.update();
    let hmd_params = hmd.gen_cfg();
    let (h_scr_w, h_scr_h) = hmd_params.scr_res;
    scrw = h_scr_w;
    scrh = h_scr_h;
    println!("HMD scrw res {}", scrw);
    println!("HMD scrh res {}", scrh);

    //Creating playerlist
    let mut playerlist: HashMap<u32, player::Player>  = HashMap::with_capacity(128);
    //Opening window
    println!("Opening window...");
    let mut window = Window::new(scrw,scrh, "test", &vrmode);

    let (display, events_loop) = window.get_display();
    //Building shaders
    println!("Building shaders...");
    let program = glium::Program::from_source(&display.display, &render::SHADER_SIMPLE_VERT, &render::SHADER_SIMPLE_FRAG, None).unwrap();
    println!("Building OHMD distortion correction shader...");
    let ohmd_dis_shaders = glium::Program::from_source(&display.display, &render::SHADER_DISTORTION_VERT, &render::SHADER_DISTORTION_FRAG, None).unwrap();

    println!("Done!");
    //Loading some assets
    let mesh_buffer = support::obj_model_loader::gen_buffer(&display.display);
    let texture_buffer = support::texture_loader::gen_texture_buffer(&display.display);
    let render_obj_buffer: HashMap<u32, render::RenderObject> = HashMap::with_capacity(1024);

    //Creating buffers and other stuff
    let mut render_data = render::RenderData{
        mesh_buf: mesh_buffer,
        texture_buf: texture_buffer,
        render_obj_buf: render_obj_buffer,
    };

    let mut camera = render::camera::Camera::new();
    //gamepad stuff
    let mut gilrs = Gilrs::new().unwrap();

    let mut vr_gui = vr_gui::VRGui::new();
    let test_element = vr_gui::Element::new((3.0,0.0), (1.0,2.0), (false, "./assets/textures/test.png".to_string()), vr_gui::ElementType::Panel);
    let test_element2 = vr_gui::Element::new((0.0,0.0), (1.0,1.0), (false, "./assets/textures/test.png".to_string()), vr_gui::ElementType::Panel);
    vr_gui.push_element(test_element);
    vr_gui.push_element(test_element2);

    let mut settings_active = false;

    //Generating random player position
    let posx = rand::thread_rng().gen_range(-7.0, 7.0);
    let posy = 0.0;
    let posz = rand::thread_rng().gen_range(-7.0, 7.0);

    let mut local_player = player::LocalPlayer::new((posx,posy,posz));

    // Audio stuff
    thread::spawn(move || {
        audio::start_audio_capture(&tx_netsound_in);
    });
    thread::spawn(move || {
        audio::start_audio_playback(&rx_netsound_out, &rx_players);
    });

    //Starting main loop
    loop{
        let sys_time = SystemTime::now();
        hmd.context.update();
        let map_objects = rx_mapobj.try_iter();
        for x in map_objects{
            let mut new_object = render::RenderObject{
                mesh_name: x.model.clone(),
                tex_name: x.texture.clone(),
                position: x.position,
                rotation: x.rotation,
                size: (1.0, 1.0, 1.0),
                visible: true
            };
            println!("{:?}", x);
            render_data.render_obj_buf.insert(rand::thread_rng().gen_range(10000, 900000), new_object);
        }
        let (posx, posy, posz) = local_player.position;

        let ohmd_orient = hmd.device.getf(openhmd_rs::ohmd_float_value::OHMD_ROTATION_QUAT);
        let quat = UnitQuaternion::from_quaternion(Quaternion::new(-ohmd_orient[0], ohmd_orient[1], ohmd_orient[2], -ohmd_orient[3]));

        local_player.rotation = (quat[0],quat[1],quat[2],quat[3]);

        gameplay::update(&mut gilrs, &mut local_player, &mut render_data, &quat);

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
            let _ = tx_players.send(playerlist.clone());
        }

        vr_gui.update(&mut render_data, local_player.position);

        camera.set_pos(nalgebra::Vector3::new(posx,posy,posz));

        //Render
        display.draw(&render_data, &program, &ohmd_dis_shaders, &hmd.device, &camera, (scrw, scrh), &vrmode, &hmd_params);

        let _ = tx_orient.send(((quat[0],quat[1],quat[2],quat[3]), (posx,posy,posz)));
        let elapsed = sys_time.elapsed().unwrap();
        /*
        let fps = 1000.0 / (((elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64) as f32 + 0.01);
        println!("FPS: {}", fps as u32);*/
    }
}
