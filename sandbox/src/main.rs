use engine::core::application::{Application,ApplicationStruct};
use engine::core::entry_point;
use engine::core::logger_wrapper;
use engine::events::application_event::*;
use engine::events::event::Event;


struct SandboxApplication{
    application_struct:ApplicationStruct,
}

impl Application for SandboxApplication {
 
    fn new() -> Self where Self: Sized {
        let window: engine::platforms::windows::WindowsWindow = engine::window::Window::create_window(engine::window::WindowProperties::new("Sandbox", 1280, 720));
        let app = ApplicationStruct {
            m_window: Box::new(window),
            m_running: true,
        };
        SandboxApplication {
            application_struct: app,
        }

        
    }
    fn run(&mut self) {
        logger_wrapper::log_info("Running application");
        while self.application_struct.m_running {
            self.application_struct.m_window.on_update();
        }
    }
}

#[no_mangle]
pub extern fn create_application() -> Box<dyn Application> {
    logger_wrapper::init_logger();

    logger_wrapper::log_info("Creating application");
    let app = SandboxApplication::new(); // Instantiate the SandboxApplication struct
    Box::new(app)
    

}

pub fn main() {

       entry_point::entry_main();
    
}



