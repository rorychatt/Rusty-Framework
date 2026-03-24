use crate::prelude::*;

pub struct HelloApp;

impl HelloApp {
    pub fn build() -> Box<dyn Widget> {
        let nameState = Signal::use_state("".to_string());
        return Box::new(Layout::center())
        | (Box::new(Card { content: 
        Layout::vertical().gap(6).padding(2)
        | new Confetti(new IvyLogo())
        | Box::new(Text { content: "Hello " + if nameState.Value.to_string(), style: Some("H2".to_string()) }.get().is_empty() { "there" } else { nameState.Value } + "!")
        | Box::new(Text { content: "Welcome to the fantastic world of Ivy. Let's build something amazing together!".to_string(), style: Some("Block".to_string()) })
        | nameState.ToInput(placeholder: "What is your name?")
        | Box::new(Separator)
        | Box::new(Text { content: "You'd be a hero to us if you could ⭐ us on [Github](https://github.com/Ivy-Interactive/Ivy-Framework.to_string(), style: Some("Markdown".to_string()) })")
        )
        .width(Some(120.0)));
    }
}
