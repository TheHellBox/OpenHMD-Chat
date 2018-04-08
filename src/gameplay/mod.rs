pub mod controls;

use gilrs;
use render;
use player;
use ncollide;
use nalgebra;

use nalgebra::geometry::{Quaternion, UnitQuaternion};
use nalgebra::core::Vector3;
use gilrs::{Event, EventType};
use render::window::RenderMode;
use render::{camera};
use glium::glutin::EventsLoop;

pub fn update(gamepad: &mut gilrs::Gilrs, local_player: &mut player::LocalPlayer, render_data: &mut render::RenderData, orient: &UnitQuaternion<f32>,
                dbvt: &mut ncollide::partitioning::DBVT<nalgebra::Point3<f32>, (nalgebra::Isometry3<f32>, (f32, f32, f32)), ncollide::bounding_volume::BoundingSphere<nalgebra::Point3<f32>>>,
                ev_loop: &mut EventsLoop){
    let matrix = UnitQuaternion::from_quaternion(Quaternion::new(local_player.rotation.0, local_player.rotation.1,local_player.rotation.2,local_player.rotation.3)).to_homogeneous();
    controls::move_player(gamepad, local_player, ev_loop);

    if (local_player.player_speed_f == 0.0) & (local_player.player_speed_lr == 0.0){
        local_player.player_moving = false;
        local_player.camera_position = local_player.position;
        local_player.rotation = (orient[0], orient[1], orient[2], orient[3]);
    }
    else{
        local_player.player_moving = true;
    }
    //Moving player
    if local_player.player_moving {

        let posx_ghost = -matrix[8] * local_player.player_speed_f + matrix[0] * local_player.player_speed_lr;
        let posz_ghost = -matrix[10] * local_player.player_speed_f + matrix[2] * local_player.player_speed_lr;

        let mut collector = Vec::new();
        {
            let ray = ncollide::query::Ray::new(nalgebra::Point3::new(local_player.position.0, local_player.position.1, local_player.position.2), -nalgebra::Vector3::y());
            let mut visitor = ncollide::query::RayInterferencesCollector::new(&ray, &mut collector);
            dbvt.visit(&mut visitor);
        }
        let ghost_rot_next = UnitQuaternion::look_at_lh(&Vector3::new(posz_ghost * 2.0, 0.0, posx_ghost * 2.0), &Vector3::new(0.0,-1.0,0.0)) * UnitQuaternion::from_euler_angles(0.0,1.570796,0.0);
        let ghost_rot = (ghost_rot_next[0], ghost_rot_next[1], ghost_rot_next[2], ghost_rot_next[3]);

        local_player.position.1 = {
            let x = collector.last();
            if x.is_some(){
                x.unwrap().0.translation.vector[1] + (x.unwrap().1).1 + 1.0
            }
            else{
                0.0
            }
        };
        local_player.position.0 += posx_ghost;
        local_player.position.2 += posz_ghost;

        let ghost = render::RenderObject{
            mesh_name: "./assets/models/player.obj".to_string(),
            tex_name: "./assets/textures/test.png".to_string(),
            position: local_player.position,
            rotation: ghost_rot,
            scale: (1.0, 1.0, 1.0),
            visible: true
        };
        render_data.render_obj_buf.insert(11119, ghost);
    }
    else{
        render_data.render_obj_buf.remove(&(11119 as u32));
    }
}
