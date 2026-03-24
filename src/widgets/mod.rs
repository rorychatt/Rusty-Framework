pub mod text;
pub mod layout;
pub mod card;
pub mod misc;

pub use text::Text;
pub use layout::Layout;
pub use card::Card;
pub use misc::{Separator, Logo, Confetti};

pub trait Widget: std::fmt::Debug {
    fn serialize(&self) -> serde_json::Value;
}
