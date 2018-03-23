use player;
use gilrs;
use gilrs::{Event, EventType};

pub fn move_player(gamepad: &mut gilrs::Gilrs, local_player: &mut player::LocalPlayer){
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
}
