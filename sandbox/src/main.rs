use engine::core::application::{Application,ApplicationStruct, WindowProps};


// struct SandboxApplication{
//     application_struct:ApplicationStruct,
// }

// impl Application for SandboxApplication {
 
//     fn new() -> Self where Self: Sized {
//         let window: engine::window::Window = engine::window::Window::new();

//         SandboxApplication {
//             application_struct:ApplicationStruct {
//                 m_window: window,
//                 m_running: true,
//             },
//         }

        
//     }
//     fn run(self) {
//         logger_wrapper::log_info("Running application");
//         while self.application_struct.m_running {
//             self.application_struct.m_window.run();
   
//         }
//     }
// }

// #[no_mangle]
// pub extern fn create_application() -> Box<dyn Application> {
//     logger_wrapper::init_logger();

//     logger_wrapper::log_info("Creating application");
//     let app = SandboxApplication::new(); // Instantiate the SandboxApplication struct
//     Box::new(app)
    

// }

pub fn main() {

    let window_prop=WindowProps {
        title: "Sandbox".to_string(),
        width: 800.0,
        height: 600.0,
        resizable: true,
        fullscreen: false,
    };
    ApplicationStruct::new(window_prop).run();    
}



