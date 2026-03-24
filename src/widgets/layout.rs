use crate::widgets::Widget;

#[derive(Debug, PartialEq)]
pub enum LayoutKind {
    Horizontal,
    Vertical,
    Grid { columns: u32 },
}

#[derive(Debug)]
pub struct Layout {
    pub kind: LayoutKind,
    pub children: Vec<Box<dyn Widget>>,
    pub centering: bool,
    pub gap: f32,
    pub padding: f32,
    pub width: Option<f32>,
}

impl Layout {
    pub fn center() -> Self {
        Self { kind: LayoutKind::Vertical, children: vec![], centering: true, gap: 4.0, padding: 0.0, width: None }
    }
    pub fn vertical() -> Self {
        Self { kind: LayoutKind::Vertical, children: vec![], centering: false, gap: 4.0, padding: 0.0, width: None }
    }
    pub fn horizontal() -> Self {
        Self { kind: LayoutKind::Horizontal, children: vec![], centering: false, gap: 4.0, padding: 0.0, width: None }
    }
    pub fn grid() -> Self {
        Self { kind: LayoutKind::Grid { columns: 1 }, children: vec![], centering: false, gap: 4.0, padding: 0.0, width: None }
    }
    pub fn columns(mut self, columns: u32) -> Self {
        self.kind = LayoutKind::Grid { columns };
        self
    }
    pub fn gap(mut self, gap: f32) -> Self { self.gap = gap; self }
    pub fn padding(mut self, padding: f32) -> Self { self.padding = padding; self }
    pub fn width(mut self, width: f32) -> Self { self.width = Some(width); self }
}

impl Widget for Layout {
    fn serialize(&self) -> serde_json::Value {
        match self.kind {
            LayoutKind::Grid { columns } => {
                serde_json::json!({
                    "type": "Ivy.GridLayout",
                    "id": uuid::Uuid::new_v4().to_string(),
                    "props": {
                        "columns": columns,
                        "rowGap": self.gap,
                        "columnGap": self.gap,
                        "padding": format!("{},{},{},{}", self.padding, self.padding, self.padding, self.padding),
                        "width": self.width.as_ref().map(|w| format!("Units:{}", w))
                    },
                    "events": [],
                    "children": self.children.iter().map(|c| c.serialize()).collect::<Vec<_>>()
                })
            }
            _ => {
                serde_json::json!({
                    "type": "Ivy.StackLayout",
                    "id": uuid::Uuid::new_v4().to_string(),
                    "props": {
                        "orientation": match self.kind {
                            LayoutKind::Horizontal => "Horizontal",
                            _ => "Vertical",
                        },
                        "align": if self.centering { Some("Center") } else { None },
                        "rowGap": self.gap,
                        "columnGap": self.gap,
                        "padding": format!("{},{},{},{}", self.padding, self.padding, self.padding, self.padding),
                        "width": self.width.as_ref().map(|w| format!("Units:{}", w))
                    },
                    "events": [],
                    "children": self.children.iter().map(|c| c.serialize()).collect::<Vec<_>>()
                })
            }
        }
    }
}

impl std::ops::BitOr<Box<dyn Widget>> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: Box<dyn Widget>) -> Self::Output {
        self.children.push(rhs);
        self
    }
}

// Helper trait to allow BitOr with any Widget
pub trait IntoBoxedWidget {
    fn into_boxed(self) -> Box<dyn Widget>;
}

impl<T: Widget + 'static> IntoBoxedWidget for T {
    fn into_boxed(self) -> Box<dyn Widget> {
        Box::new(self)
    }
}

impl<T: Widget + 'static> std::ops::BitOr<T> for Layout {
    type Output = Layout;
    fn bitor(mut self, rhs: T) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}

impl From<Layout> for Box<dyn Widget> {
    fn from(l: Layout) -> Self {
        Box::new(l)
    }
}
