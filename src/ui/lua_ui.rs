use hlua;
use scripting_engine;
use hlua::AnyLuaValue;
use std::collections::HashMap;
use scripting_engine::{LuaEvent, LUA_CHANNL_OUT};

pub struct LuaRawButton{
    pub text: String,
    pub position:  (f64, f64),
    pub size:      (f64, f64),
    pub callback: String
}

impl LuaRawButton {
    pub fn new(text: String, position: (f64, f64), callback: String) -> LuaRawButton{
        LuaRawButton{
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
    pub fn add_button(&mut self, button: LuaRawButton, id: u32){
        self.buttons.insert(id, button);
    }
}

pub struct LuaButton{
    pub id: u32
}
implement_lua_read!(LuaButton);
implement_lua_push!(LuaButton, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("set_size", hlua::function3(|btn: &mut LuaButton, x: f64, y: f64|
    {
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(LuaEvent::ChangeButtonSize(btn.id, (x, y)));
    } ));
});
