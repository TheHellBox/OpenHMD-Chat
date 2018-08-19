#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate conrod;
#[macro_use]
extern crate hlua;

extern crate rand;
extern crate tobj;
extern crate clap;
extern crate opus;
extern crate alto;
extern crate serde;
extern crate image;
extern crate cobalt;
extern crate bincode;
extern crate nalgebra;
extern crate openhmd_rs;

mod ui;
mod game;
mod audio;
mod render;
mod network;
mod support;
mod scripting_engine;

use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion};
use std::sync::{Arc, Mutex};
use std::{thread, time, process};
use clap::{Arg, App};


fn main() {
    println!("Hello, world!");
    let matches = App::new("OpenHMD-Chat")
        .version("0.1")
        .author("The HellBox <thehellbox11@gmail.com>")
        .about("Online chat for VR")
        .arg(Arg::with_name("addr")
            .short("a")
            .long("addr")
            .help("Sets addr to connect to")
            .takes_value(true))
        .arg(Arg::with_name("vr")
            .short("v")
            .long("vr")
            .help("VR mode"))
        .get_matches();

    println!("Main thread ID {}", process::id());
    let mut vr_mode = false;

    let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4460").to_string();
    let vr = matches.values_of_lossy("vr");
    if vr.is_some(){
        vr_mode = true;
    }

    let frames = 1280;
    let sample_rate = 16000;

    let mut network = network::Network::new();
    let net_tx = network.tx.clone();

    let audio = audio::AudioWrapper::new().unwrap();
    let tx_audio = audio.init(sample_rate, frames, net_tx);

    thread::spawn(move || {
        network.connect(addr);
        network.init(tx_audio.clone());
    });

    // For fixed update, I know that I can do that in main thread, but...
    let ticks = Arc::new(Mutex::new(0));
    let c_ticks = ticks.clone();
    thread::spawn(move || {
        loop{
            *c_ticks.lock().unwrap() += 1;
            thread::sleep(time::Duration::from_millis(16));
        }
    });

    let mut window = render::Window::new(1024, 768, "OpenHMD-Chat", vr_mode);
    window.init();

    let mut game = game::Game::new();
    println!("Running lua...");
    let mut scripting_eng = scripting_engine::ScriptingEngine::new();
    println!("Done!");
    //Move it to lua side
    let test_model = window.load_model("./assets/models/scene/scene.obj".to_string());

    let mut ui = ui::Ui::new(&window.display, window.scr_res);

    window.add_draw_object("scene_01".to_string(), test_model,
        Point3::new(0.0, 0.0, 0.0),
        UnitQuaternion::from_quaternion(Quaternion::new(0.707, 0.0, 0.707, 0.0)),
        (0.1, 0.1, 0.1),
        "simple");

    let ui_sphere = window.load_model("./assets/models/cube/cube.obj".to_string());
    let gui_scale = (window.scr_res.0 as f32 / 20000.0, window.scr_res.1 as f32 / 20000.0);
    window.add_draw_object("ui_renderer".to_string(), ui_sphere,
        Point3::new(0.0, 0.0, 0.0),
        UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        (gui_scale.0, gui_scale.1, 0.1),
        "solid");

    let gui_gm = game::gameobject::GameObjectBuilder::new()
        .with_name("gui_gm".to_string())
        .with_position(Point3::new(0.0, 0.0, 0.0))
        .with_rotation_unit(UnitQuaternion::from_euler_angles(0.0, -90.0, 0.0))
        .with_render_object("ui_renderer".to_string())
        .build();

    game.spawn_game_object(gui_gm);

    loop{
        {
            let ui_renderer = &mut window.draw_buffer.objects.get_mut("ui_renderer").unwrap().model.meshes[0];
            if let Some(tex) = ui.draw_into_texture(&window.display, window.scr_res){
                ui_renderer.texture = tex;
            }
        }
        game.update();
        for _ in 0..*ticks.lock().unwrap(){
            game.fixed_update();
        }
        *ticks.lock().unwrap() = 0;

        scripting_eng.update(&mut game);
        ui.update(&mut window);
        window.draw(&game);
        window.update();
        thread::sleep(time::Duration::from_millis(1));
    }
}
