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
    pub color1_button: &'static str,
    pub color2_button: &'static str,
    pub color3_button: &'static str,
    pub color4_button: &'static str,
    pub color5_button: &'static str,
    pub color6_button: &'static str,
    pub color7_button: &'static str,
    pub color8_button: &'static str,
    pub color9_button: &'static str,
    pub color10_button: &'static str,
    pub color11_button: &'static str,
    pub color12_button: &'static str,
    pub color13_button: &'static str,
    pub color14_button: &'static str,
    pub color15_button: &'static str,
    pub color16_button: &'static str,
}

pub fn themes() -> Vec<Theme> {
    vec![
        Theme::default(),
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
            ..Default::default()
        },
    ]
}

impl Default for Theme {
    fn default() -> Self {
        Self {
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
            color1_button: "
            aria-pressed:bg-red-600
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color2_button: "
            aria-pressed:bg-pink-500
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color3_button: "
            aria-pressed:bg-fuchsia-400
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color4_button: "
            aria-pressed:bg-green-600
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color5_button: "
            aria-pressed:bg-emerald-500
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color6_button: "
            aria-pressed:bg-amber-200
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color7_button: "
            aria-pressed:bg-amber-200
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color8_button: "
            aria-pressed:bg-yellow-400
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color9_button: "
            aria-pressed:bg-purple-600
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color10_button: "
            aria-pressed:bg-fuchsia-400
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color11_button: "
            aria-pressed:bg-fuchsia-400
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color12_button: "
            aria-pressed:bg-fuchsia-400
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color13_button: "
            aria-pressed:bg-blue-400
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color14_button: "
            aria-pressed:bg-teal-300
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color15_button: "
            aria-pressed:bg-aqua-200
            aria-pressed:text-white aria-pressed:stroke-white
        ",
            color16_button: "
            aria-pressed:bg-cyan-500
            aria-pressed:text-white aria-pressed:stroke-white
        ",
        }
    }
}
