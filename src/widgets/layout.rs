use crate::widgets::{Widget, Text, Card, Logo, Separator, Confetti};

#[derive(Debug)]
pub struct Layout {
    pub children: Vec<Box<dyn Widget>>,
    pub centering: bool,
    pub gap: f32,
    pub padding: f32,
    pub width: Option<f32>,
}

impl Layout {
    pub fn center() -> Self {
        Self { children: vec![], centering: true, gap: 0.0, padding: 0.0, width: None }
    }
    pub fn vertical() -> Self {
        Self { children: vec![], centering: false, gap: 0.0, padding: 0.0, width: None }
    }
    pub fn gap(mut self, gap: f32) -> Self { self.gap = gap; self }
    pub fn padding(mut self, padding: f32) -> Self { self.padding = padding; self }
    pub fn width(mut self, width: f32) -> Self { self.width = Some(width); self }
}

impl Widget for Layout {
    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "layout",
            "centering": self.centering,
            "gap": self.gap,
            "padding": self.padding,
            "width": self.width,
            "children": self.children.iter().map(|c| c.serialize()).collect::<Vec<_>>()
        })
    }
}

impl std::ops::BitOr<Box<dyn Widget>> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: Box<dyn Widget>) -> Self::Output {
        self.children.push(rhs);
        self
    }
}

impl std::ops::BitOr<Text> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: Text) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}

impl std::ops::BitOr<Card> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: Card) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}

impl std::ops::BitOr<Logo> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: Logo) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}

impl std::ops::BitOr<Separator> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: Separator) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}

impl std::ops::BitOr<Confetti> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: Confetti) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}

// Convert Layout to Box<dyn Widget>
impl From<Layout> for Box<dyn Widget> {
    fn from(l: Layout) -> Self {
        Box::new(l)
    }
}
