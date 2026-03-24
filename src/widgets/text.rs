use crate::widgets::Widget;

#[derive(Debug)]
pub struct Text {
    pub content: String,
    pub kind: String,
}

impl Text {
    pub fn h1(content: String) -> Self { Self { content, kind: "H1".to_string() } }
    pub fn h2(content: String) -> Self { Self { content, kind: "H2".to_string() } }
    pub fn h3(content: String) -> Self { Self { content, kind: "H3".to_string() } }
    pub fn p(content: String) -> Self { Self { content, kind: "P".to_string() } }
    pub fn monospaced(content: String) -> Self { Self { content, kind: "Monospaced".to_string() } }
    pub fn block(content: String) -> Self { Self { content, kind: "Block".to_string() } }
    pub fn markdown(content: String) -> Self { Self { content, kind: "Markdown".to_string() } }
}

impl Widget for Text {
    fn serialize(&self) -> serde_json::Value {
        if self.kind == "Markdown" {
            return serde_json::json!({
                "type": "Ivy.Markdown",
                "id": uuid::Uuid::new_v4().to_string(),
                "props": {
                    "content": self.content
                },
                "events": [],
                "children": []
            });
        }
        serde_json::json!({
            "type": "Ivy.TextBlock",
            "id": uuid::Uuid::new_v4().to_string(),
            "props": {
                "content": self.content,
                "variant": self.kind
            },
            "events": [],
            "children": []
        })
    }
}
