use crate::window;

pub struct ApplicationStruct{
    pub m_window: Box<dyn window::Window>,
    pub m_running: bool,
}
pub trait Application {
  
    fn new() -> Self where Self: Sized;
    fn run(&mut self);
  
}

