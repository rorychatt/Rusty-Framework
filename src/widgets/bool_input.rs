use crate::widgets::Widget;
use crate::core::signal::Signal;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum BoolInputVariant {
    Checkbox,
    Switch,
    Toggle,
}

#[derive(Debug)]
pub struct BoolInput {
    pub signal: Signal<bool>,
    pub variant: BoolInputVariant,
    pub label: Option<String>,
    pub description: Option<String>,
    pub disabled: bool,
    pub density: String,
    pub on_change: Option<String>,
    pub invalid: Option<String>,
}

impl BoolInput {
    pub fn new(signal: Signal<bool>) -> Self {
        Self {
            signal,
            variant: BoolInputVariant::Checkbox,
            label: None,
            description: None,
            disabled: false,
            density: "Medium".to_string(),
            on_change: None,
            invalid: None,
        }
    }

    pub fn variant(mut self, variant: BoolInputVariant) -> Self { self.variant = variant; self }
    pub fn label(mut self, label: String) -> Self { self.label = Some(label); self }
    pub fn description(mut self, description: String) -> Self { self.description = Some(description); self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
    pub fn density(mut self, density: String) -> Self { self.density = density; self }
    pub fn small(mut self) -> Self { self.density = "Small".to_string(); self }
    pub fn large(mut self) -> Self { self.density = "Large".to_string(); self }
    pub fn on_change(mut self, id: String) -> Self { self.on_change = Some(id); self }
    pub fn invalid(mut self, invalid: String) -> Self { self.invalid = Some(invalid); self }
}

impl Widget for BoolInput {
    fn serialize(&self) -> Value {
        json!({
            "type": "Ivy.BoolInput",
            "id": self.signal.id(),
            "props": {
                "value": self.signal.get(),
                "variant": self.variant,
                "label": self.label,
                "description": self.description,
                "disabled": self.disabled,
                "density": self.density,
                "onChange": self.on_change,
                "invalid": self.invalid,
            },
            "events": ["OnChange"],
            "children": []
        })
    }
}
