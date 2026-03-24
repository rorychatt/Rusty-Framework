use serde::{Serialize, Deserialize};
use serde_json::Value;

pub trait Widget: std::fmt::Debug + Send + Sync {
    fn serialize(&self) -> Value;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[derive(Debug)]
pub struct Text {
    pub content: String,
    pub style: Option<String>,
}

impl Widget for Text {
    fn serialize(&self) -> Value {
        serde_json::json!({
            "type": "Text",
            "content": self.content,
            "style": self.style
        })
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

#[derive(Debug)]
pub struct Card {
    pub content: Box<dyn Widget>,
    pub width: Option<f32>,
}

impl Widget for Card {
    fn serialize(&self) -> Value {
        serde_json::json!({
            "type": "Card",
            "content": self.content.serialize(),
            "width": self.width
        })
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Separator;

impl Widget for Separator {
    fn serialize(&self) -> Value {
        serde_json::json!({ "type": "Separator" })
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

#[derive(Debug)]
pub struct Logo;

impl Widget for Logo {
    fn serialize(&self) -> Value {
        serde_json::json!({ "type": "Logo" })
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

#[derive(Debug)]
pub struct Confetti {
    pub child: Box<dyn Widget>,
}

impl Widget for Confetti {
    fn serialize(&self) -> Value {
        serde_json::json!({
            "type": "Confetti",
            "child": self.child.serialize()
        })
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

#[derive(Debug)]
pub struct Layout {
    pub children: Vec<Box<dyn Widget>>,
    pub layout_type: String,
    pub gap: f32,
    pub padding: f32,
}

impl Widget for Layout {
    fn serialize(&self) -> Value {
        let children: Vec<Value> = self.children.iter().map(|c| c.serialize()).collect();
        serde_json::json!({
            "type": "Layout",
            "layout_type": self.layout_type,
            "gap": self.gap,
            "padding": self.padding,
            "children": children
        })
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

impl Layout {
    pub fn center() -> Self {
        Self {
            children: Vec::new(),
            layout_type: "Center".to_string(),
            gap: 0.0,
            padding: 0.0,
        }
    }

    pub fn vertical() -> Self {
        Self {
            children: Vec::new(),
            layout_type: "Vertical".to_string(),
            gap: 0.0,
            padding: 0.0,
        }
    }

    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn add<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
    }
}

// Emulate the pipe operator | for adding children
impl std::ops::BitOr<Box<dyn Widget>> for Layout {
    type Output = Layout;

    fn bitor(mut self, rhs: Box<dyn Widget>) -> Self::Output {
        self.children.push(rhs);
        self
    }
}
