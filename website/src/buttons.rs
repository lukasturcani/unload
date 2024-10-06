use dioxus::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Primary,
    Secondary,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Size {
    #[allow(dead_code)]
    Small,
    Medium,
    Large,
}

#[component]
pub fn ButtonLink(
    href: ReadOnlySignal<String>,
    color: Option<Color>,
    size: Option<Size>,
    children: Element,
) -> Element {
    let color = color.unwrap_or(Color::Primary);
    let size = size.unwrap_or(Size::Medium);
    let text_color = match color {
        Color::Primary => "text-text-primary",
        Color::Secondary => "text-text-primary",
    };
    let bg_color = match color {
        Color::Primary => "bg-primary-main hover:bg-primary-dark",
        Color::Secondary => "bg-secondary-main hover:bg-secondary-light",
    };
    let border = match color {
        Color::Primary => "",
        Color::Secondary => "border border-gray-600",
    };
    let focus = match color {
        Color::Primary => "focus:ring-focus",
        Color::Secondary => "focus:ring-gray-700",
    };
    let size = match size {
        Size::Small => "text-xs px-3 py-2",
        Size::Medium => "text-sm px-5 py-2.5",
        Size::Large => "text-base px-6 py-3.5",
    };
    rsx! {
        a {
            href,
            class: "{text_color} {bg_color} {border} {focus} {size} \
                text-center focus:ring-4 font-medium rounded-lg focus:outline-none",
            {children}
        }
    }
}
