pub mod text;
pub mod layout;
pub mod card;
pub mod misc;
pub mod input;

pub use text::Text;
pub use layout::Layout;
pub use card::Card;
pub use misc::{Separator, Logo, Confetti};
pub use input::TextInput;

pub trait Widget: std::fmt::Debug + Send + Sync {
    fn serialize(&self) -> serde_json::Value;
}
