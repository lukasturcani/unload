use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SavedTheme(pub String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub text_color: &'static str,
    pub action_text_color: &'static str,
    pub focus_color: &'static str,
    pub late_text_color: &'static str,
    pub border_color: &'static str,
    pub late_border_color: &'static str,
    pub active_border_color: &'static str,
    pub bg_color_1: &'static str,
    pub bg_color_2: &'static str,
    pub divide_color: &'static str,
    pub hover_color: &'static str,
    pub button: &'static str,
    pub primary_button: &'static str,
    pub color1_button: &'static str,
    pub color1_text: &'static str,
    pub color2_button: &'static str,
    pub color2_text: &'static str,
    pub color3_button: &'static str,
    pub color3_text: &'static str,
    pub color4_button: &'static str,
    pub color4_text: &'static str,
    pub color5_button: &'static str,
    pub color5_text: &'static str,
    pub color6_button: &'static str,
    pub color6_text: &'static str,
    pub color7_button: &'static str,
    pub color7_text: &'static str,
    pub color8_button: &'static str,
    pub color8_text: &'static str,
    pub color9_button: &'static str,
    pub color9_text: &'static str,
    pub color10_button: &'static str,
    pub color10_text: &'static str,
    pub color11_button: &'static str,
    pub color11_text: &'static str,
    pub color12_button: &'static str,
    pub color12_text: &'static str,
    pub color13_button: &'static str,
    pub color13_text: &'static str,
    pub color14_button: &'static str,
    pub color14_text: &'static str,
    pub color15_button: &'static str,
    pub color15_text: &'static str,
    pub color16_button: &'static str,
    pub color16_text: &'static str,
}

pub fn themes() -> Vec<Theme> {
    vec![
        Theme {
            name: "Default (Dynamic)",
            text_color: "
                text-gray-900 stroke-gray-900 accent-purple-600
                dark:text-white dark:stroke-white
            ",
            focus_color: "
                focus:outline-none focus:ring-2
                focus:ring-purple-600 focus:border-purple-600
                dark:focus:ring-blue-500 dark:focus:border-blue-500
            ",
            late_text_color: "
                text-red-500 stroke-red-500
                dark:text-red-600 dark:stroke-red-600
            ",
            border_color: "
                border-gray-400
                dark:border-gray-700
            ",
            late_border_color: "
                border-x-2 border-x-red-500 border-y-pink-400 sm:border-red-500
                dark:border-x-2 dark:border-x-red-600 dark:border-y-gray-700 dark:sm:border-red-600
            ",
            active_border_color: "
                border-white
                dark:border-gray-900
            ",
            bg_color_1: "
                bg-white
                dark:bg-gray-900
            ",
            bg_color_2: "
                bg-white
                dark:bg-gray-800
            ",
            divide_color: "
                divide-gray-400
                dark:divide-gray-700
            ",
            button: "
                border-gray-900
                aria-pressed:bg-gray-900
                aria-pressed:text-white aria-pressed:stroke-white

                dark:border-white
                dark:aria-pressed:bg-white
                dark:aria-pressed:text-black dark:aria-pressed:stroke-black
            ",
            primary_button: "
                text-white bg-purple-600
                active:bg-purple-700 sm:hover:bg-purple-700

                dark:bg-blue-600
                dark:active:bg-blue-700 dark:sm:hover:bg-blue-700
            ",
            hover_color: "
                hover:bg-purple-600 hover:text-white
                dark:hover:bg-blue-600
            ",
            ..Default::default()
        },
        Theme::default(),
        Theme {
            name: "Default (Light)",
            text_color: "text-gray-900 stroke-gray-900 accent-purple-600",
            focus_color: "
                focus:outline-none focus:ring-2
                focus:ring-purple-600 focus:border-purple-600
            ",
            late_text_color: "text-red-500 stroke-red-500",
            border_color: "border-gray-400",
            late_border_color: "
                border-x-2 border-x-red-500 border-y-gray-400 sm:border-red-500
            ",
            active_border_color: "border-gray-900",
            bg_color_1: "bg-white",
            bg_color_2: "bg-white",
            divide_color: "divide-gray-400",
            button: "
                border-gray-900
                aria-pressed:bg-gray-900
                aria-pressed:text-white aria-pressed:stroke-white
            ",
            primary_button: "
                text-white bg-purple-600
                active:bg-purple-700 sm:hover:bg-purple-700
            ",
            hover_color: "hover:bg-purple-600 hover:text-white",
            ..Default::default()
        },
    ]
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Default (Dark)",
            text_color: "text-white stroke-white",
            action_text_color: "text-blue-500",
            focus_color: "
                focus:outline-none focus:ring-2
                focus:ring-blue-500 focus:border-blue-500
            ",
            late_text_color: "text-red-600 stroke-red-600",
            border_color: "border-gray-700",
            late_border_color: "
                border-x-2 border-x-red-600 border-y-gray-700 sm:border-red-600
            ",
            active_border_color: "border-white",
            bg_color_1: "bg-gray-900",
            bg_color_2: "bg-gray-800",
            divide_color: "divide-gray-700",
            hover_color: "hover:bg-blue-600",
            button: "
                aria-pressed:bg-white
                aria-pressed:text-black aria-pressed:stroke-black
            ",
            primary_button: "
                bg-blue-600
                active:bg-blue-700 sm:hover:bg-blue-700
            ",
            color1_button: "
                border-orange-500
                aria-pressed:bg-orange-500
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-orange-500
                group-[.text-colored]:text-orange-500
            ",
            color1_text: "text-orange-500",
            color2_button: "
                border-violet-400
                aria-pressed:bg-violet-400
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-violet-400
                group-[.text-colored]:text-violet-400
            ",
            color2_text: "text-violet-400",
            color3_button: "
                border-gray-400
                aria-pressed:bg-gray-400
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-gray-400
                group-[.text-colored]:text-gray-400
            ",
            color3_text: "text-gray-400",
            color4_button: "
                border-slate-500
                aria-pressed:bg-slate-500
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-slate-500
                group-[.text-colored]:text-slate-500
            ",
            color4_text: "text-slate-500",
            color5_button: "
                border-rose-400
                aria-pressed:bg-rose-400
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-rose-400
                group-[.text-colored]:text-rose-400
            ",
            color5_text: "text-rose-400",
            color6_button: "
                border-red-600
                aria-pressed:bg-red-600
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-red-600
                group-[.text-colored]:text-red-600
            ",
            color6_text: "text-red-600",
            color7_button: "
                border-purple-600
                aria-pressed:bg-purple-600
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-purple-600
                group-[.text-colored]:text-purple-600
            ",
            color7_text: "text-purple-600",
            color8_button: "
                border-fuchsia-400
                aria-pressed:bg-fuchsia-400
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-fuchsia-400
                group-[.text-colored]:text-fuchsia-400
            ",
            color8_text: "text-fuchsia-400",
            color9_button: "
                border-emerald-500
                aria-pressed:bg-emerald-500
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-emerald-500
                group-[.text-colored]:text-emerald-500
            ",
            color9_text: "text-emerald-500",
            color10_button: "
                border-lime-500
                aria-pressed:bg-lime-500
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-lime-500
                group-[.text-colored]:text-lime-500
            ",
            color10_text: "text-lime-500",
            color11_button: "
                border-indigo-400
                aria-pressed:bg-indigo-400
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-indigo-400
                group-[.text-colored]:text-indigo-400
            ",
            color11_text: "text-indigo-400",
            color12_button: "
                border-yellow-400
                aria-pressed:bg-yellow-400
                aria-pressed:text-black aria-pressed:stroke-black
                group-[.filled]:bg-yellow-400
                group-[.text-colored]:text-yellow-400
            ",
            color12_text: "text-yellow-400",
            color13_button: "
                border-amber-200
                aria-pressed:bg-amber-200
                aria-pressed:text-black aria-pressed:stroke-black
                group-[.filled]:bg-amber-200
                group-[.text-colored]:text-amber-200
            ",
            color13_text: "text-amber-200",
            color14_button: "
                border-blue-400
                aria-pressed:bg-blue-400
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-blue-400
                group-[.text-colored]:text-blue-400
            ",
            color14_text: "text-blue-400",
            color15_button: "
                border-teal-300
                aria-pressed:bg-teal-300
                aria-pressed:text-black aria-pressed:stroke-black
                group-[.filled]:bg-teal-300
                group-[.text-colored]:text-teal-300
            ",
            color15_text: "text-teal-300",
            color16_button: "
                border-cyan-500
                aria-pressed:bg-cyan-500
                aria-pressed:text-white aria-pressed:stroke-white
                group-[.filled]:bg-cyan-500
                group-[.text-colored]:text-cyan-500
            ",
            color16_text: "text-cyan-500",
        }
    }
}
