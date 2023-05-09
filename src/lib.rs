pub mod app;
pub mod buffer;
pub mod callback;
pub mod platform;
pub mod style;
pub mod vec2;
pub mod widget;
pub mod widget2;
pub mod widgets;
pub mod widgets2;

mod draw_buffer;

pub mod prelude {
    pub use crate::platform::event::*;
    pub use crate::style::*;
    pub use crate::vec2::Vec2;
    pub use crate::widget2::app::*;
    pub use crate::widget2::widget::{Handled, *};
    pub use crate::widgets2::*;
}
