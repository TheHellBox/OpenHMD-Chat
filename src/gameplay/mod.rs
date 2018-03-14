use gilrs;
use render;
use player;
use nalgebra::geometry::UnitQuaternion;
use nalgebra::geometry::Quaternion;
use std::collections::HashMap;
use gilrs::{Gilrs, Button, Event, EventType};
use render::window::RenderMode;

pub fn update(gamepad: &mut gilrs::Gilrs, local_player: &mut player::LocalPlayer, render_data: &mut render::RenderData, orient: &UnitQuaternion<f32>){
    let matrix = orient.to_homogeneous();
    while let Some(event) = gamepad.next_event() {
        match event {
            Event { id, event: EventType::AxisChanged(gilrs::ev::Axis::LeftStickY, val1, val2), .. } => {
                if val1 > 0.1{
                    local_player.player_speed_f = 0.1 * val1;
                }
                else if val1 < -0.1{
                    local_player.player_speed_f = 0.1 * val1;
                }
                else if (val1 > -0.1) & (val1 < 0.1){
                    local_player.player_speed_f = 0.0;
                }

            }
            Event { id, event: EventType::AxisChanged(gilrs::ev::Axis::LeftStickX, val1, val2), .. } => {
                if val1 > 0.1{
                    local_player.player_speed_lr = 0.1 * val1;
                }
                else if val1 < -0.1{
                    local_player.player_speed_lr = 0.1 * val1;
                }
                else if (val1 > -0.1) & (val1 < 0.1){
                    local_player.player_speed_lr = 0.0;
                }
            }
            Event { id, event: EventType::ButtonPressed(gilrs::ev::Button::Start, val1), .. } => {
                //settings_active = match settings_active{
                //    false => true,
                //    true => false,
                //};
            }
            _ => (),
        };
    }

    if (local_player.player_speed_f == 0.0) & (local_player.player_speed_lr == 0.0){
        local_player.player_moving = false;
        local_player.position = local_player.ghost_position;
    }
    else{
        local_player.player_moving = true;
    }
    //Moving player
    if local_player.player_moving {
        let (posx, posy, posz) = local_player.ghost_position;
        let posx_ghost = matrix[8] * local_player.player_speed_f + matrix[0] * local_player.player_speed_lr;
        let posz_ghost = matrix[10] * local_player.player_speed_f + matrix[2] * local_player.player_speed_lr;
        local_player.ghost_position = (posx - posx_ghost, 0.0, posz - posz_ghost);
        let mut ghost = render::RenderObject{
            mesh_name: "./assets/models/monkey.obj".to_string(),
            tex_name: "./assets/textures/test.png".to_string(),
            position: local_player.ghost_position,
            rotation: local_player.rotation,
            size: (1.0, 1.0, 1.0),
            visible: true
        };
        render_data.render_obj_buf.insert(11119, ghost);
    }
    else{
        render_data.render_obj_buf.remove(&(11119 as u32) );
    }
}
