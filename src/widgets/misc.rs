use crate::widgets::Widget;

#[derive(Debug)]
pub struct Separator;
impl Widget for Separator {
    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({ "type": "separator" })
    }
}

#[derive(Debug)]
pub struct Logo;
impl Widget for Logo {
    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({ "type": "logo" })
    }
}

#[derive(Debug)]
pub struct Confetti {
    pub item: Box<dyn Widget>,
}
impl Confetti {
    pub fn new(item: Box<dyn Widget>) -> Self {
        Self { item }
    }
}
impl Widget for Confetti {
    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({ "type": "confetti", "child": self.item.serialize() })
    }
}
