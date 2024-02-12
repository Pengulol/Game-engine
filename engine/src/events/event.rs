
pub enum EventType {
    None,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

#[repr(u32)]
pub enum EventCategory {
   None = 0,
   EventCategoryApplication = 1<<0,
   EventCategoryInput = 1<<1,
   EventCategoryKeyboard = 1<<2,
   EventCategoryMouse = 1<<3,
   EventCategoryMouseButton = 1<<4,
}


impl std::ops::BitOr for EventCategory {
    type Output = u32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    } 
}

macro_rules! EVENT_CLASS_TYPE {
    ($type:ident) => {
        fn get_static_type(&self) -> EventType {
            return EventType::$type;
        }
        fn get_event_type(&self) -> EventType {
            return Self::get_static_type(self);
        }
        fn get_name(&self) -> &'static str {
            return stringify!($type);
        }
    };
}

pub(crate) use EVENT_CLASS_TYPE;

macro_rules! EVENT_CLASS_CATEGORY {
    ($category:expr) => {
        fn get_category_flags(&self) -> u32 {
            return $category as u32;

        }
    };

}
pub(crate) use EVENT_CLASS_CATEGORY;

pub trait Event {
    fn get_static_type(&self) -> EventType;
    fn get_event_type(&self) -> EventType;
    fn get_name(&self) -> &'static str;
    fn get_category_flags(&self) -> u32;
    fn is_in_category(&self,category: EventCategory) -> bool{
        return (self.get_category_flags() & category as u32) != 0;
    }
    fn to_string_name(&self) -> &'static str{
        return self.get_name();
    }


}
pub struct EventStruct{
    pub handled: bool
}
impl EventStruct {
    pub fn new() -> EventStruct {
        EventStruct {
            handled: false,
        }
    }

}

// impl Event for EventStruct {
//     fn get_event_type() -> EventType {
//         return EventType::None;
//     }
//     fn get_name() -> &'static str {
//         return "None";
//     }
//     fn get_category_flags() -> u32 {
//         return 0;
//     }
//     fn to_string_name() -> &'static str {
//         return "None";
//     }

//     fn is_in_category(category: EventCategory) -> bool {
//         return (Self::get_category_flags() & category as u32) != 0;
//     }

//     fn get_static_type() -> EventType {
//         return EventType::None;
//     }
// }


// pub trait EventDispatcher<T,F> {
//     fn dispatch(&self, func: F) -> bool
//     where
//         T: Event,
//         F: Fn(&T) -> bool;
// }

// pub struct EventDispatcherStruct
// {
//     event: dyn Event,

// }

// impl EventDispatcher<T,F> for EventDispatcherStruct{
//     fn dispatch(&self, func: F) -> bool
    
//     {
//         if self.event.get_event_type() == T::get_static_type() {
//             self.event.handled = func(&self.event);
//             return true;
//         }
//         return false;
//     }
       
// }

// impl EventDispatcherStruct {
//     pub fn new(event: dyn Event) -> EventDispatcherStruct {
//         EventDispatcherStruct {
//             event: event,
//         }
//     }
// }