use rusty_framework::prelude::*;

fn main() {
    let name_state = Signal::use_state("".to_string());

    // Build the "Hello" template logic
    let name = name_state.get();
    let name_display = if name.is_empty() { "there" } else { &name };

    let app = Layout::center()
        | Box::new(Card {
            content: Box::new(Layout::vertical().gap(6.0).padding(2.0)
                | Box::new(Confetti { child: Box::new(Logo) })
                | Box::new(Text {
                    content: format!("Hello {}!", name_display),
                    style: Some("H2".to_string()),
                })
                | Box::new(Text {
                    content: "Welcome to the fantastic world of Ivy. Let's build something amazing together!".to_string(),
                    style: Some("Block".to_string()),
                })
                | Box::new(Separator)
                | Box::new(Text {
                    content: "You'd be a hero to us if you could ⭐ us on Github".to_string(),
                    style: Some("Markdown".to_string()),
                })
            ),
            width: Some(500.0),
        });

    println!("--- RUSTY IVY DEMO ---");
    println!("Serialized UI:");
    println!("{}", serde_json::to_string_pretty(&app.serialize()).unwrap());
    
    // Simulate a state change
    println!("\nChanging state to 'Rory'...");
    name_state.set("Rory".to_string());
    
    // In a real app, this would trigger a rebuild and re-render
    let name_updated = name_state.get();
    let name_display_updated = if name_updated.is_empty() { "there" } else { &name_updated };

    let app_updated = Layout::center()
        | Box::new(Card {
            content: Box::new(Layout::vertical().gap(6.0).padding(2.0)
                | Box::new(Confetti { child: Box::new(Logo) })
                | Box::new(Text {
                    content: format!("Hello {}!", name_display_updated),
                    style: Some("H2".to_string()),
                })
            ),
            width: Some(500.0),
        });
    
    println!("Updated Serialized UI (partial):");
    println!("{}", serde_json::to_string_pretty(&app_updated.serialize()).unwrap());
}
