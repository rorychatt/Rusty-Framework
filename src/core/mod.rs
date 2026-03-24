pub mod signal;
pub use signal::Signal;
use crate::widgets::Widget;

pub trait IvyApp: Send + Sync {
    fn build(&self) -> Box<dyn Widget>;
    fn update_state(&self, signal_id: &str, value: &str);
}
