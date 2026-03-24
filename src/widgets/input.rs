use crate::widgets::Widget;
use crate::core::signal::Signal;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct TextInput {
    pub signal: Signal<String>,
    pub placeholder: String,
    pub variant: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub disabled: bool,
    pub invalid: Option<String>,
    pub density: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub shortcut_key: Option<String>,
    pub on_change: Option<String>,
    pub on_blur: Option<String>,
}

impl TextInput {
    pub fn new(signal: Signal<String>) -> Self {
        Self {
            signal,
            placeholder: String::new(),
            variant: "Text".to_string(),
            label: None,
            description: None,
            disabled: false,
            invalid: None,
            density: "Medium".to_string(),
            prefix: None,
            suffix: None,
            min_length: None,
            max_length: None,
            shortcut_key: None,
            on_change: None,
            on_blur: None,
        }
    }

    pub fn placeholder(mut self, placeholder: String) -> Self { self.placeholder = placeholder; self }
    pub fn variant(mut self, variant: String) -> Self { self.variant = variant; self }
    pub fn label(mut self, label: String) -> Self { self.label = Some(label); self }
    pub fn description(mut self, description: String) -> Self { self.description = Some(description); self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
    pub fn invalid(mut self, invalid: String) -> Self { self.invalid = Some(invalid); self }
    pub fn density(mut self, density: String) -> Self { self.density = density; self }
    pub fn small(mut self) -> Self { self.density = "Small".to_string(); self }
    pub fn large(mut self) -> Self { self.density = "Large".to_string(); self }
    pub fn prefix(mut self, prefix: String) -> Self { self.prefix = Some(prefix); self }
    pub fn suffix(mut self, suffix: String) -> Self { self.suffix = Some(suffix); self }
    pub fn min_length(mut self, min: u32) -> Self { self.min_length = Some(min); self }
    pub fn max_length(mut self, max: u32) -> Self { self.max_length = Some(max); self }
    pub fn shortcut_key(mut self, key: String) -> Self { self.shortcut_key = Some(key); self }
    pub fn on_change(mut self, id: String) -> Self { self.on_change = Some(id); self }
    pub fn on_blur(mut self, id: String) -> Self { self.on_blur = Some(id); self }
    
    // Lowcase aliases for transpiler
    pub fn shortcutkey(self, key: String) -> Self { self.shortcut_key(key) }
}

impl Widget for TextInput {
    fn serialize(&self) -> Value {
        json!({
            "type": "Ivy.TextInput",
            "id": self.signal.id(),
            "props": {
                "value": self.signal.get(),
                "placeholder": self.placeholder,
                "variant": self.variant,
                "label": self.label,
                "description": self.description,
                "disabled": self.disabled,
                "invalid": self.invalid,
                "density": self.density,
                "prefix": self.prefix,
                "suffix": self.suffix,
                "minLength": self.min_length,
                "maxLength": self.max_length,
                "shortcutKey": self.shortcut_key,
                "onChange": self.on_change,
                "onBlur": self.on_blur,
            },
            "events": ["OnChange"],
            "children": []
        })
    }
}
