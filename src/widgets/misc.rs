use crate::widgets::Widget;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub enum LogoType {
    Ivy,
}

#[derive(Debug)]
pub struct Separator;
impl Separator {
    pub fn new() -> Self { Self }
}
impl Widget for Separator {
    fn serialize(&self) -> Value {
        json!({ 
            "type": "Ivy.Separator",
            "id": uuid::Uuid::new_v4().to_string(),
            "props": {},
            "events": [],
            "children": []
        })
    }
}

#[derive(Debug)]
pub struct Logo;
impl Logo {
    pub fn new(_kind: LogoType) -> Self {
        Self
    }
}
impl Widget for Logo {
    fn serialize(&self) -> Value {
        json!({ 
            "type": "Ivy.IvyLogo",
            "id": uuid::Uuid::new_v4().to_string(),
            "props": {},
            "events": [],
            "children": []
        })
    }
}

#[derive(Debug)]
pub struct Confetti {
    pub child: Box<dyn Widget>,
}
impl Confetti {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self { child }
    }
}

impl Widget for Confetti {
    fn serialize(&self) -> Value {
        json!({
            "type": "Ivy.Confetti",
            "id": uuid::Uuid::new_v4().to_string(),
            "props": {},
            "events": [],
            "children": [self.child.serialize()]
        })
    }
}

#[derive(Debug)]
pub struct Spacer {
    pub width: Option<f32>,
    pub height: Option<f32>,
}
impl Spacer {
    pub fn new() -> Self { Self { width: None, height: None } }
    pub fn width(mut self, w: f32) -> Self { self.width = Some(w); self }
    pub fn height(mut self, h: f32) -> Self { self.height = Some(h); self }
}
impl Widget for Spacer {
    fn serialize(&self) -> Value {
        json!({
            "type": "Ivy.Spacer",
            "id": uuid::Uuid::new_v4().to_string(),
            "props": {
                "width": self.width.map(|w| format!("Units:{}", w)),
                "height": self.height.map(|h| format!("Units:{}", h)),
            },
            "events": [],
            "children": []
        })
    }
}

#[derive(Debug)]
pub struct Badge {
    pub content: String,
}
impl Badge {
    pub fn new(content: String) -> Self { Self { content } }
}
impl Widget for Badge {
    fn serialize(&self) -> Value {
        json!({
            "type": "Ivy.Badge",
            "id": uuid::Uuid::new_v4().to_string(),
            "props": {
                "content": self.content
            },
            "events": [],
            "children": []
        })
    }
}
