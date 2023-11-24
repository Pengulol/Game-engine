pub trait Application {
  
    fn new() -> Self where Self: Sized;
    fn run(&self);
  
}

