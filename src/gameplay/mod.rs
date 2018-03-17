use gilrs;
use render;
use player;
use nalgebra::geometry::{Quaternion, UnitQuaternion};
use nalgebra::core::Vector3;
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

    let (posx1, posy1, posz1) = local_player.position;
    let (posx2, posy2, posz2) = local_player.ghost_position;
    let (posr_x,posr_y,posr_z) = ((posx2 + local_player.player_speed_f * 100.0) - posx1, posy2 - posy1, (posz2 + local_player.player_speed_lr * 100.0) - posz1);
    let ghost_rot_next = UnitQuaternion::look_at_lh(&Vector3::new(-posr_x, 0.0, posr_z), &Vector3::new(0.0,-1.0,0.0));
    local_player.ghost_rotation = (ghost_rot_next[0], ghost_rot_next[1], ghost_rot_next[2], ghost_rot_next[3]);
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
            rotation: local_player.ghost_rotation,
            size: (1.0, 1.0, 1.0),
            visible: true
        };
        render_data.render_obj_buf.insert(11119, ghost);
    }
    else{
        render_data.render_obj_buf.remove(&(11119 as u32) );
    }
}
