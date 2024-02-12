use super::event::*;


pub struct KeyPressedEventStruct{
    pub event: EventStruct,
    repeat_count: i32,
    key_code: i32,
}



pub trait KeyPressedEvent{
    EVENT_CLASS_TYPE!(KeyPressed);
    EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryKeyboard | EventCategory::EventCategoryInput);
    fn get_repeat_count(&self) -> i32;
    fn get_key_code(&self) -> i32;


}

impl KeyPressedEvent for KeyPressedEventStruct {
    fn get_repeat_count(&self) -> i32 {
        return self.repeat_count;
    }
    fn get_key_code(&self) -> i32 {
        return self.key_code;
    }

}

impl KeyPressedEventStruct{
    fn new(event: EventStruct, repeat_count: i32, key_code: i32) -> KeyPressedEventStruct {
        return KeyPressedEventStruct {
            event: event,
            repeat_count: repeat_count,
            key_code: key_code,
        };
    }

}


pub struct KeyReleasedEventStruct{
    event: EventStruct,
    key_code: i32,
}

pub trait KeyReleasedEvent{
    EVENT_CLASS_TYPE!(KeyReleased);
    EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryKeyboard | EventCategory::EventCategoryInput);
    fn get_key_code(&self) -> i32;
}

impl KeyReleasedEvent for KeyReleasedEventStruct {
    fn get_key_code(&self) -> i32 {
        return self.key_code;
    }
}

impl KeyReleasedEventStruct{
    fn new(event: EventStruct, key_code: i32) -> KeyReleasedEventStruct {
        return KeyReleasedEventStruct {
            event: event,
            key_code: key_code,
        };
    }
}





 