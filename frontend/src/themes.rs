#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub text_color: &'static str,
    pub late_text_color: &'static str,
    pub border_color: &'static str,
    pub late_border_color: &'static str,
    pub bg_color_1: &'static str,
    pub bg_color_2: &'static str,
    pub divide_color: &'static str,
    pub button: &'static str,
}

pub const THEMES: &[Theme] = &[
    Theme {
        name: "Default - Dark",
        text_color: "text-white stroke-white",
        late_text_color: "text-red-600 stroke-red-600",
        border_color: "border-gray-700",
        late_border_color: "border-red-600",
        bg_color_1: "bg-gray-900",
        bg_color_2: "bg-gray-800",
        divide_color: "divide-gray-700",
        button: "
            aria-pressed:bg-white
            aria-pressed:text-black aria-pressed:stroke-black
        ",
    },
    Theme {
        name: "Pink - Dark",
        text_color: "text-white stroke-white",
        late_text_color: "text-red-300 stroke-red-300",
        border_color: "border-pink-400",
        late_border_color: "border-red-300",
        bg_color_1: "bg-gray-900",
        bg_color_2: "bg-pink-500",
        divide_color: "divide-pink-400",
        button: "
            aria-pressed:bg-white
            aria-pressed:text-black aria-pressed:stroke-black
        ",
    },
];
