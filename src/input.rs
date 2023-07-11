use std::collections::HashMap;

pub struct InputManager {
    keys: HashMap<winit::event::VirtualKeyCode, bool>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn key_pressed(&self, key: winit::event::VirtualKeyCode) -> bool {
        match self.keys.get(&key) {
            Some(&state) => state,
            None => false,
        }
    }

    pub fn set_key_state(&mut self, key: winit::event::VirtualKeyCode, state: bool) {
        self.keys.insert(key, state);
    }
}
