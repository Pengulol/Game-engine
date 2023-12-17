
use super::event::*;

/*

#pragma once

#include "Hazel/Events/Event.h"
#include "Hazel/Core/MouseCodes.h"

namespace Hazel {

	class MouseMovedEvent : public Event
	{
	public:
		MouseMovedEvent(const float x, const float y)
			: m_MouseX(x), m_MouseY(y) {}

		float GetX() const { return m_MouseX; }
		float GetY() const { return m_MouseY; }

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseMovedEvent: " << m_MouseX << ", " << m_MouseY;
			return ss.str();
		}

		EVENT_CLASS_TYPE(MouseMoved)
		EVENT_CLASS_CATEGORY(EventCategoryMouse | EventCategoryInput)
	private:
		float m_MouseX, m_MouseY;
	};

	class MouseScrolledEvent : public Event
	{
	public:
		MouseScrolledEvent(const float xOffset, const float yOffset)
			: m_XOffset(xOffset), m_YOffset(yOffset) {}

		float GetXOffset() const { return m_XOffset; }
		float GetYOffset() const { return m_YOffset; }

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseScrolledEvent: " << GetXOffset() << ", " << GetYOffset();
			return ss.str();
		}

		EVENT_CLASS_TYPE(MouseScrolled)
		EVENT_CLASS_CATEGORY(EventCategoryMouse | EventCategoryInput)
	private:
		float m_XOffset, m_YOffset;
	};

	class MouseButtonEvent : public Event
	{
	public:
		MouseCode GetMouseButton() const { return m_Button; }

		EVENT_CLASS_CATEGORY(EventCategoryMouse | EventCategoryInput | EventCategoryMouseButton)
	protected:
		MouseButtonEvent(const MouseCode button)
			: m_Button(button) {}

		MouseCode m_Button;
	};

	class MouseButtonPressedEvent : public MouseButtonEvent
	{
	public:
		MouseButtonPressedEvent(const MouseCode button)
			: MouseButtonEvent(button) {}

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseButtonPressedEvent: " << m_Button;
			return ss.str();
		}

		EVENT_CLASS_TYPE(MouseButtonPressed)
	};

	class MouseButtonReleasedEvent : public MouseButtonEvent
	{
	public:
		MouseButtonReleasedEvent(const MouseCode button)
			: MouseButtonEvent(button) {}

		std::string ToString() const override
		{
			std::stringstream ss;
			ss << "MouseButtonReleasedEvent: " << m_Button;
			return ss.str();
		}

		EVENT_CLASS_TYPE(MouseButtonReleased)
	};

}
*/

pub struct MouseMovedEventStruct {
	event: EventStruct,
	mouse_x: f32,
	mouse_y: f32,
}

pub trait MouseMovedEvent {
	EVENT_CLASS_TYPE!(MouseMoved);
	EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryMouse | EventCategory::EventCategoryInput);
	fn get_x(&self) -> f32;
	fn get_y(&self) -> f32;
}

impl MouseMovedEvent for MouseMovedEventStruct {
	fn get_x(&self) -> f32 {
		return self.mouse_x;
	}
	fn get_y(&self) -> f32 {
		return self.mouse_y;
	}
}

impl MouseMovedEventStruct {
	pub fn new(event: EventStruct, mouse_x: f32, mouse_y: f32) -> MouseMovedEventStruct {
		return MouseMovedEventStruct {
			event: event,
			mouse_x: mouse_x,
			mouse_y: mouse_y,
		};
	}
}

pub struct MouseScrolledEventStruct {
	event: EventStruct,
	x_offset: f32,
	y_offset: f32,
}

pub trait MouseScrolledEvent {
	EVENT_CLASS_TYPE!(MouseScrolled);
	EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryMouse | EventCategory::EventCategoryInput);
	fn get_x_offset(&self) -> f32;
	fn get_y_offset(&self) -> f32;
}

impl MouseScrolledEvent for MouseScrolledEventStruct {
	fn get_x_offset(&self) -> f32 {
		return self.x_offset;
	}
	fn get_y_offset(&self) -> f32 {
		return self.y_offset;
	}
}

impl MouseScrolledEventStruct {
	pub fn new(event: EventStruct, x_offset: f32, y_offset: f32) -> MouseScrolledEventStruct {
		return MouseScrolledEventStruct {
			event: event,
			x_offset: x_offset,
			y_offset: y_offset,
		};
	}
}

pub struct MouseButtonEventStruct {
	event: EventStruct,
	mouse_button: i32,
}

pub trait MouseButtonEvent {
	EVENT_CLASS_CATEGORY!(EventCategory::EventCategoryMouse | EventCategory::EventCategoryInput);
	fn get_mouse_button(&self) -> i32;
}

impl MouseButtonEvent for MouseButtonEventStruct {
	fn get_mouse_button(&self) -> i32 {
		return self.mouse_button;
	}
}

impl MouseButtonEventStruct {
	pub fn new(event: EventStruct, mouse_button: i32) -> MouseButtonEventStruct {
		return MouseButtonEventStruct {
			event: event,
			mouse_button: mouse_button,
		};
	}
}

pub struct MouseButtonPressedEventStruct {
	mouse_button_event: MouseButtonEventStruct,
	mouse_button: i32,
}

pub trait MouseButtonPressedEvent {
	EVENT_CLASS_TYPE!(MouseButtonPressed);
}

impl MouseButtonPressedEvent for MouseButtonPressedEventStruct {}

impl MouseButtonPressedEventStruct {
	pub fn new(event: EventStruct, mouse_button: i32) -> MouseButtonPressedEventStruct {
		return MouseButtonPressedEventStruct {
			mouse_button_event: MouseButtonEventStruct::new(event, mouse_button),
			mouse_button: mouse_button,
		};
	}
}