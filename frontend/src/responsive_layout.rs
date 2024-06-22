#[derive(Eq, PartialEq)]
pub enum ResponsiveLayout {
    Narrow,
    Wide,
}

impl ResponsiveLayout {
    pub fn from_window() -> Self {
        if width() < 640.0 {
            ResponsiveLayout::Narrow
        } else {
            ResponsiveLayout::Wide
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn width() -> f64 {
    web_sys::window()
        .unwrap()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn width() -> f64 {
    dioxus::mobile::window().inner_size().width as f64
}
