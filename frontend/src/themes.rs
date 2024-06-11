#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub text_color: &'static str,
    pub late_text_color: &'static str,
    pub border_color: &'static str,
    pub late_border_color: &'static str,
    pub bg_color_1: &'static str,
    pub bg_color_2: &'static str,
    pub divide_color: &'static str,
}

pub const THEMES: &[Theme] = &[
    Theme {
        text_color: "text-white stroke-white",
        late_text_color: "text-red-600",
        border_color: "border-gray-700",
        late_border_color: "border-red-600",
        bg_color_1: "bg-gray-900",
        bg_color_2: "bg-gray-800",
        divide_color: "divide-gray-700",
    },
    Theme {
        text_color: "text-white stroke-white",
        late_text_color: "text-red-600",
        border_color: "border-gray-700",
        late_border_color: "border-red-600",
        bg_color_1: "bg-gray-900",
        bg_color_2: "bg-gray-800",
        divide_color: "divide-gray-700",
    },
];
