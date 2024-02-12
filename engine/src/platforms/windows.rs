extern crate glfw;
use glfw::Context;
use crate::window::{Window,WindowProperties};
use crate::events;

//implement the window trait for the windows platform struct 
struct WindowData{
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    event_callback: Option<Box<dyn FnMut(dyn events::event::Event)>>,

}


pub struct WindowsWindow {
    window: glfw::PWindow,
    glfw:  glfw::Glfw,
    data: WindowData,

}


impl Window for WindowsWindow {
    fn create_window(windowProperties: WindowProperties) -> Self {
            let mut w=WindowsWindow::new( windowProperties);
            // unsafe { glfwGetWindowUserPointer(w.window) };
            w.set_vsync(true);
            return w;

    }

    fn on_update(&mut self) {
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

    fn get_width(&self) -> u32 {
        self.data.width
    }

    fn get_height(&self) -> u32 {
        self.data.height
    }

    fn set_event_callback(&mut self, callback: Box<dyn FnMut(dyn events::event::Event)>) {
        self.data.event_callback = Some(callback);
    }

    fn set_vsync(&mut self, enabled: bool) {
        if(enabled)
        {
            self.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        }
        else {
            self.glfw.set_swap_interval(glfw::SwapInterval::None);
        }
        self.data.vsync = enabled;
    }

    fn is_vsync(&self) -> bool {
        self.data.vsync
    }
}

impl WindowsWindow {
    pub fn new(props: WindowProperties) -> WindowsWindow {
        let mut data = WindowData {
            title: props.title,
            width: props.width,
            height: props.height,
            vsync: true,
            event_callback: None,
        };


            let mut glfw: glfw::Glfw = glfw::init_no_callbacks().unwrap();
            // assert!(success, "Could not initialize GLFW!");

            let (mut window, events) = glfw.create_window(data.width, data.height, &data.title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");
            window.make_current();

            


        WindowsWindow { window,glfw, data }
    }


    pub fn shutdown(&self) {
        //destroy window
        // unsafe { glfwDestroyWindow(self.window) }
    }


}
impl Drop for WindowsWindow {
    fn drop(&mut self) {
        self.shutdown()
    }    
}


