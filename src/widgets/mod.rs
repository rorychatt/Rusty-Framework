pub mod text;
pub mod layout;
pub mod card;
pub mod misc;
pub mod input;
pub mod bool_input;

pub use text::Text;
pub use layout::Layout;
pub use card::Card;
pub use misc::{Separator, Logo, Confetti, Spacer, Badge};
pub use input::TextInput;
pub use bool_input::{BoolInput, BoolInputVariant};

pub trait Widget: std::fmt::Debug + Send + Sync {
    fn serialize(&self) -> serde_json::Value;
}
