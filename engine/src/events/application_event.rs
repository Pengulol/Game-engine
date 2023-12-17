use super::event::*;

 pub struct WindowResizeEventStruct {
     event: EventStruct,
     width: u32,
     height: u32,
 }

    pub trait WindowResizeEvent : Event {
       
        fn get_width(&self) -> u32;
        fn get_height(&self) -> u32;
    }

    impl WindowResizeEvent for WindowResizeEventStruct {
        fn get_width(&self) -> u32 {
            return self.width;
        }
        fn get_height(&self) -> u32 {
            return self.height;
        }
 

    }
    impl Event for WindowResizeEventStruct{
        EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryApplication);
        EVENT_CLASS_TYPE!(WindowResize); 
    }
    impl WindowResizeEventStruct {
        pub fn new( width: u32, height: u32) -> WindowResizeEventStruct {
            return WindowResizeEventStruct {
                event: EventStruct::new(),
                width: width,
                height: height,
            };
        }
    }


    pub struct WindowCloseEventStruct {
        event: EventStruct,
    }

    pub trait WindowCloseEvent {
        EVENT_CLASS_TYPE!(WindowClose);
        EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryApplication);
    }

    impl WindowCloseEvent for WindowCloseEventStruct {
    }

    impl WindowCloseEventStruct {
        pub fn new(event: EventStruct) -> WindowCloseEventStruct {
            return WindowCloseEventStruct {
                event: event,
            };
        }
    }

    pub struct AppTickEventStruct {
        event: EventStruct,
    }

    pub trait AppTickEvent {
        EVENT_CLASS_TYPE!(AppTick);
        EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryApplication);
    }

    impl AppTickEvent for AppTickEventStruct {
    }

    impl AppTickEventStruct {
        pub fn new(event: EventStruct) -> AppTickEventStruct {
            return AppTickEventStruct {
                event: event,
            };
        }
    }

    pub struct AppUpdateEventStruct {
        event: EventStruct,
    }

    pub trait AppUpdateEvent {
        EVENT_CLASS_TYPE!(AppUpdate);
        EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryApplication);
    }

    impl AppUpdateEvent for AppUpdateEventStruct {
    }

    impl AppUpdateEventStruct {
        pub fn new(event: EventStruct) -> AppUpdateEventStruct {
            return AppUpdateEventStruct {
                event: event,
            };
        }
    }

    pub struct AppRenderEventStruct {
        event: EventStruct,
    }

    pub trait AppRenderEvent {
        EVENT_CLASS_TYPE!(AppRender);
        EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryApplication);
    }


    impl AppRenderEvent for AppRenderEventStruct {
    }

    impl AppRenderEventStruct {
        pub fn new(event: EventStruct) -> AppRenderEventStruct {
            return AppRenderEventStruct {
                event: event,
            };
        }
    }

