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
    pub use crate::style::Style;
    pub use crate::widget::Handled;
    pub use crate::widget2::app::*;
    pub use crate::widget2::widget::*;
    //
    pub use crate::widgets2::container::{Container, LineStyle};
    pub use crate::widgets2::label::Label;
}
