use std::collections::HashMap;
use rand;
use rand::Rng;

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
    pub size: (f32, f32),
    pub container: (bool, String),
    pub el_type: ElementType,
    pub visible: bool,
    pub rend_id: u32
}

impl Element{
    pub fn new(pos: (f32, f32), size: (f32, f32), container: (bool, String), el_type: ElementType) -> Element{
        Element{
            position: pos,
            size: size,
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

    pub fn set_size(&mut self, size: (f32, f32)){
        self.size = size;
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
    pub fn set_element_size(&mut self, id: u32, size: (f32, f32)){
        let element = self.elements.get_mut(&id);
        if element.is_some(){
            let element = element.unwrap();
            element.set_size(size);
        }
        else{
            println!("Failed to set element size");
        }
    }
    pub fn remove_element(&mut self, id: u32){
        let element = self.elements.remove(&id);
    }
}
