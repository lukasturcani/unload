#[derive(Eq, PartialEq)]
pub enum ResponsiveLayout {
    Narrow,
    Wide,
}

impl ResponsiveLayout {
    pub fn from_window() -> Self {
        let width = web_sys::window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap();
        if width < 640.0 {
            ResponsiveLayout::Narrow
        } else {
            ResponsiveLayout::Wide
        }
    }
}
