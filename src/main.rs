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

extern crate alga;
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

use nalgebra::geometry::{Point3, UnitQuaternion};
use std::sync::{Arc, Mutex};
use std::{thread, time, process};
use clap::{Arg, App};


fn main() {
    println!("Hello, world!");

    let settings = game::settings::Settings::new();

    let frames = 2048;
    let sample_rate = 16000;

    let (mut network, mut net_rx) = network::Network::new();
    let mut net_tx = network.tx_in.clone();
    let ip = settings.server_ip.clone();

    let audio = audio::AudioWrapper::new().unwrap();
    let tx_audio = audio.init(sample_rate, frames, net_tx.clone());

    thread::spawn(move || {
        network.connect(ip);
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

    let mut window = render::Window::new("OpenHMD-Chat", &settings);
    window.init();

    let mut game = game::Game::new();
    println!("Running lua...");
    let mut scripting_eng = scripting_engine::ScriptingEngine::new();
    println!("Done!");
    //Move it to lua side
    let mut ui = ui::Ui::new(&window.display, window.scr_res);
    let gui_scale = (window.scr_res.0 as f32 / 20000.0, window.scr_res.1 as f32 / 20000.0);

    //Load models
    let _ = window.load_model_and_push("./assets/models/ui_plane/ui_plane.obj".to_string(), "ui_plane".to_string(), (0.1, 0.1, 0.1));
    // Create game objects
    let gui_go = game::gameobject::GameObjectBuilder::new()
        .with_name("gui_go".to_string())
        .with_position(Point3::new(0.0, 0.7, 0.0))
        .with_scale((0.1, gui_scale.1, gui_scale.0))
        .with_rotation_unit(UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0))
        .with_render_object("ui_plane".to_string())
        .build();

    // Spawn them
    game.spawn_game_object(gui_go);

    loop{
        {
            let ui_renderer = &mut window.draw_buffer.objects.get_mut("ui_plane").unwrap().model.meshes[0];
            if let Some(tex) = ui.draw_into_texture(&window.display, window.scr_res){
                ui_renderer.texture = tex;
            }
        }
        game.update(&mut net_rx, &mut net_tx, &mut window);
        for _ in 0..*ticks.lock().unwrap(){
            game.fixed_update();
        }
        *ticks.lock().unwrap() = 0;

        scripting_eng.update(&mut game, &mut window);
        ui.update(&mut window);
        window.draw(&game);
        window.update();
        thread::sleep(time::Duration::from_millis(1));
    }
}
