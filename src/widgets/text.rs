use crate::widgets::Widget;

#[derive(Debug)]
pub struct Text {
    pub content: String,
    pub kind: String,
}

impl Text {
    pub fn h2(content: String) -> Self {
        Self { content, kind: "h2".to_string() }
    }
    pub fn block(content: String) -> Self {
        Self { content, kind: "block".to_string() }
    }
    pub fn markdown(content: String) -> Self {
        Self { content, kind: "markdown".to_string() }
    }
}

impl Widget for Text {
    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "text",
            "kind": self.kind,
            "content": self.content
        })
    }
}
