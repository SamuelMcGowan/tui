pub mod app;
pub mod buffer;
pub mod callback;
pub mod platform;
pub mod style;
pub mod vec2;
pub mod widget;
pub mod widgets;

mod draw_buffer;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::platform::event::*;
    pub use crate::style::*;
    pub use crate::vec2::*;
    pub use crate::widget::*;
    pub use crate::widgets::*;
}
