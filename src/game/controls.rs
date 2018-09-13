use render::Window;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use glium::glutin::{Event, WindowEvent, VirtualKeyCode, ElementState};

pub fn update_controls(window: &mut Window){
    use support::direction;
    for event in window.events.clone(){
        match event{
            Event::WindowEvent{window_id: _, event: window_event} => {
                match window_event{
                    WindowEvent::KeyboardInput{device_id: _, input} => {
                        let key = input.virtual_keycode;
                        let state = match input.state{
                            ElementState::Pressed => true,
                            _ => false
                        };
                        if let Some(key) = key{
                            match key{
                                VirtualKeyCode::Q => {
                                    if !state {
                                        let mut rotation = UnitQuaternion::from_euler_angles(0.0, -0.7853982, 0.0);
                                        window.character_view.rotation *= rotation;
                                    }
                                },
                                VirtualKeyCode::E => {
                                    if !state {
                                        let mut rotation = UnitQuaternion::from_euler_angles(0.0, 0.7853982, 0.0);
                                        window.character_view.rotation *= rotation;
                                    }
                                },
                                VirtualKeyCode::W => {
                                    if !state {
                                        window.character_view.position.vector -= direction(window.head_dir, Vector3::new(0.0, 0.0, 1.0)) / 3.0;
                                        let position = window.character_view.position.vector;
                                        let position = Point3::new(position[0], position[1], position[2]);
                                    }
                                },
                                VirtualKeyCode::S => {
                                    if !state {
                                        window.character_view.position.vector += direction(window.head_dir, Vector3::new(0.0, 0.0, 1.0)) / 3.0;
                                        let position = window.character_view.position.vector;
                                        let position = Point3::new(position[0], position[1], position[2]);
                                    }
                                },
                                VirtualKeyCode::A => {
                                    if !state {
                                        window.character_view.position.vector -= direction(window.head_dir, Vector3::new(1.0, 0.0, 0.0)) / 3.0;
                                        let position = window.character_view.position.vector;
                                        let position = Point3::new(position[0], position[1], position[2]);
                                    }
                                },
                                VirtualKeyCode::D => {
                                    if !state {
                                        window.character_view.position.vector += direction(window.head_dir, Vector3::new(1.0, 0.0, 0.0)) / 3.0;
                                        let position = window.character_view.position.vector;
                                        let position = Point3::new(position[0], position[1], position[2]);
                                    }
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}
