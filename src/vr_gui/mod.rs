use std::collections::HashMap;
use rand;
use rand::Rng;
use render;

#[derive(PartialEq)]
pub enum ElementType{
    Button,
    Text,
    Panel,
    CheckBox
}

pub struct VRGui{
    pub elements: HashMap<u32, Element>
}

#[derive(PartialEq)]
pub struct Element{
    pub position: (f32, f32),
    pub scale: (f32, f32),
    pub container: (bool, String),
    pub el_type: ElementType,
    pub visible: bool,
    pub rend_id: u32
}

impl Element{
    pub fn new(pos: (f32, f32), scale: (f32, f32), container: (bool, String), el_type: ElementType) -> Element{
        Element{
            position: pos,
            scale: scale,
            container: container,
            el_type: el_type,
            visible: true,
            rend_id: rand::thread_rng().gen_range(0, 10000)
        }
    }

    pub fn modify(&mut self, container: (bool, String)){
        self.container = container;
    }

    pub fn set_position(&mut self, pos: (f32, f32)){
        self.position = pos;
    }

    pub fn set_scale(&mut self, scale: (f32, f32)){
        self.scale = scale;
    }
}

impl VRGui{
    pub fn new() -> VRGui{
        VRGui{
            elements: HashMap::new()
        }
    }
    pub fn push_element(&mut self, element: Element){
        let len = self.elements.len();
        self.elements.insert(len as u32, element);
    }
    pub fn modify_element(&mut self, id: u32, container: (bool, String)){
        let element = self.elements.get_mut(&id);
        if element.is_some(){
            let element = element.unwrap();
            element.modify(container);
        }
        else{
            println!("Failed to modify element");
        }
    }
    pub fn set_element_position(&mut self, id: u32, pos: (f32, f32)){
        let element = self.elements.get_mut(&id);
        if element.is_some(){
            let element = element.unwrap();
            element.set_position(pos);
        }
        else{
            println!("Failed to set element position");
        }
    }
    pub fn set_element_scale(&mut self, id: u32, scale: (f32, f32)){
        let element = self.elements.get_mut(&id);
        if element.is_some(){
            let element = element.unwrap();
            element.set_scale(scale);
        }
        else{
            println!("Failed to set element size");
        }
    }
    pub fn remove_element(&mut self, id: u32){
        let element = self.elements.remove(&id);
    }
    pub fn update(&mut self, render_data: &mut render::RenderData, player_pos: (f32, f32, f32)){
        let (posx, posy, posz) = player_pos;
        for (_, x) in &self.elements{
            if x.el_type == self::ElementType::Panel {
                let (gui_posx, gui_posy) = x.position;
                let (gui_sizex, gui_sizey) = x.scale;
                let (_, texture) = x.container.clone();
                let object = render::RenderObject{
                    mesh_name: "./assets/models/cube.obj".to_string(),
                    tex_name: texture,
                    position: (-posx + gui_posx,-posy + gui_posy,-posz - 2.0),
                    rotation: (0.0, 0.0, 1.0, 0.0),
                    scale: (gui_sizex, gui_sizey, 0.1),
                    visible: true
                };
                render_data.render_obj_buf.insert(x.rend_id, object);
            }
        }
    }
}
