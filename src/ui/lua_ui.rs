use hlua;
use scripting_engine;
use hlua::AnyLuaValue;
use std::collections::HashMap;
use scripting_engine::{LuaEvent, LUA_CHANNL_OUT};

#[derive(Clone)]
pub struct LuaRawButton{
    pub id: u32,
    pub text: String,
    pub position:  (f64, f64),
    pub size:      (f64, f64),
    pub callback: String
}

impl LuaRawButton {
    pub fn new(text: String, position: (f64, f64), callback: String) -> LuaRawButton{
        LuaRawButton{
            id: 0,
            text,
            position,
            size: (128.0, 128.0),
            callback
        }
    }
    pub fn press(&self){
        let channels = scripting_engine::LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(scripting_engine::LuaEvent::CallEvent(self.callback.clone(), vec![]));
    }
    pub fn set_size(&mut self, size: (f64, f64)){
        self.size = size;
    }
}

pub struct LuaUI{
    pub buttons: HashMap<u32, LuaRawButton>
}

impl LuaUI {
    pub fn new() -> LuaUI{
        LuaUI{
            buttons: HashMap::new()
        }
    }
    pub fn add_button(&mut self, button: LuaRawButton){
        self.buttons.insert(button.id, button);
    }
}

implement_lua_read!(LuaRawButton);
implement_lua_push!(LuaRawButton, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("SetSize", hlua::function3(|btn: &mut LuaRawButton, x: f64, y: f64|
    {
        btn.size = (x, y);
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(LuaEvent::UpdateButton(btn.clone()));
    } ));
    index.set("SetPosition", hlua::function3(|btn: &mut LuaRawButton, x: f64, y: f64|
    {
        btn.position = (x, y);
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(LuaEvent::UpdateButton(btn.clone()));
    } ));
    index.set("SetLabel", hlua::function2(|btn: &mut LuaRawButton, label: String|
    {
        btn.text = label;
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(LuaEvent::UpdateButton(btn.clone()));
    } ));
});
