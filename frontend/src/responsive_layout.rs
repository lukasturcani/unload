use crate::window::WindowSize;

#[derive(Eq, PartialEq)]
pub enum ResponsiveLayout {
    Narrow,
    Wide,
}

impl ResponsiveLayout {
    pub fn from_window_size(window_size: WindowSize) -> Self {
        if window_size.width < 640 {
            ResponsiveLayout::Narrow
        } else {
            ResponsiveLayout::Wide
        }
    }
}
