
//import core and events modules
use crate::events;


pub struct WindowProperties {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl WindowProperties {
    //constructor
    //give title width and height default values
    pub fn new(title: &str, width: u32, height: u32) -> WindowProperties {
        WindowProperties {
            title: title.to_string(),
            width,
            height,
        }
    }
}

impl Default for WindowProperties {
    fn default() -> Self {
        WindowProperties {
            title: "Engine".to_string(),
            width: 1280,
            height: 720,
        }
    }
}



pub trait Window {
    fn on_update(&mut self);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn set_event_callback(&mut self, callback: Box<dyn FnMut(dyn events::event::Event)>);
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
    fn create_window(windowProperties: WindowProperties) -> Self where Self: Sized;
}