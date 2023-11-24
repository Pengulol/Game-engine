use engine::core::application::Application;
use engine::core::entry_point;
use engine::core::logger_wrapper;

struct SandboxApplication;

impl Application for SandboxApplication {
 
    fn new() -> Self where Self: Sized {
        SandboxApplication
    }
    fn run(&self) {
        loop {}
    }
}

#[no_mangle]
pub extern fn create_application() -> Box<dyn Application> {
    logger_wrapper::init_logger();

    logger_wrapper::log_info("Creating application");
    let app = SandboxApplication;
    Box::new(app)
    

}

pub fn main() {

       entry_point::entry_main();
    
}



