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
        let mut children = Vec::new();
        if let Some(child) = &self.child {
            children.push(serde_json::json!({
                "type": "Ivy.Slot",
                "id": uuid::Uuid::new_v4().to_string(),
                "props": {
                    "name": "Content"
                },
                "events": [],
                "children": [child.serialize()]
            }));
        }

        serde_json::json!({
            "type": "Ivy.Card",
            "id": uuid::Uuid::new_v4().to_string(),
            "props": {
                "width": self.width.as_ref().map(|w| format!("Units:{}", w))
            },
            "events": [],
            "children": children
        })
    }
}
