use crate::window;
use crate::core::logger_wrapper;

#[derive(Debug, Clone, PartialEq)]
pub struct WindowProps {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub resizable: bool,
    pub fullscreen: bool,
}
impl Default for WindowProps {
    fn default() -> Self {
        WindowProps {
            title: "Rust Engine".to_string(),
            width: 800.0,
            height: 600.0,
            resizable: true,
            fullscreen: false,
        }
    }
    
}

pub struct ApplicationStruct{
    pub m_window: window::Window,
}
pub trait Application {
  
    fn new(window_prop : WindowProps) -> Self;
    fn run(self);
  
}
impl Application for ApplicationStruct {
  
    fn new(window_prop : WindowProps) -> Self {
        logger_wrapper::init_logger();
        logger_wrapper::log_info("Creating application");
        
        let window: window::Window = window::Window::new(window_prop);

        ApplicationStruct {
            m_window: window,
        }
    }
    fn run(self) {
        logger_wrapper::log_info("Running application");
        self.m_window.run();
        
    }
    
}

