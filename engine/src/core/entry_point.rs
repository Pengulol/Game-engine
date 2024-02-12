
use crate::core::application::Application;
//implemented in the sandbox
extern  {
    pub fn create_application() -> Box<dyn Application>;
    }
//entry point for the engine    
#[no_mangle]
pub fn entry_main() {
    unsafe {
        let mut app = create_application();
        (*app).run();
    }

}