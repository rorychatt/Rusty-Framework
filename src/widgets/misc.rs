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
