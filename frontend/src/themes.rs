#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub text_color: &'static str,
    pub border_color: &'static str,
    pub bg_color_1: &'static str,
    pub bg_color_2: &'static str,
    pub active_bg_color_2: &'static str,
    pub sm_hover_bg_color_2: &'static str,
    pub sm_hover_bg_color_3: &'static str,
    pub active_bg_color_3: &'static str,
    pub divide_color: &'static str,
}

pub const THEMES: &[Theme] = &[Theme {
    text_color: "text-white stroke-white",
    border_color: "border-gray-700",
    bg_color_1: "bg-gray-900",
    bg_color_2: "bg-gray-800",
    active_bg_color_2: "active:bg-gray-800",
    sm_hover_bg_color_2: "sm:hover:bg-gray-800",
    sm_hover_bg_color_3: "sm:hover:bg-gray-700",
    active_bg_color_3: "active:bg-gray-700",
    divide_color: "divide-gray-700",
}];
