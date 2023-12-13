use crossterm::style::{ContentStyle, Stylize};

pub const DEF_STYLE: fn() -> ContentStyle = || ContentStyle::new();
pub const ERROR_STYLE: fn() -> ContentStyle = || ContentStyle::new().with(crossterm::style::Color::Red).bold();
pub const _INFO_STYLE: fn() -> ContentStyle = || ContentStyle::new().with(crossterm::style::Color::Green).bold();
