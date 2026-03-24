use crate::widgets::Widget;

#[derive(Debug)]
pub struct Card {
    pub child: Option<Box<dyn Widget>>,
    pub width: Option<f32>,
}

impl Card {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self { child: Some(child), width: None }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for Card {
    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "card",
            "width": self.width,
            "child": self.child.as_ref().map(|c| c.serialize())
        })
    }
}
