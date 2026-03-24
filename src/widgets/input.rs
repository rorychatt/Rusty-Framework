use crate::widgets::Widget;
use crate::core::signal::Signal;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct TextInput {
    pub signal: Signal<String>,
    pub placeholder: String,
}

impl TextInput {
    pub fn new(signal: Signal<String>) -> Self {
        Self {
            signal,
            placeholder: String::new(),
        }
    }

    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }
}

impl Widget for TextInput {
    fn serialize(&self) -> Value {
        json!({
            "type": "Ivy.TextInput",
            "id": self.signal.id(),
            "props": {
                "value": self.signal.get(),
                "placeholder": self.placeholder,
            },
            "events": ["OnChange"],
            "children": []
        })
    }
}
