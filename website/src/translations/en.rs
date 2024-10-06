use super::{Text, Translation};

pub const EN: Translation<&'static str> = Translation {
    id: "en",
    name: "EN - English",
    text: Text {
        open_main_menu: "Open main menu",
        home_section_label: "Home",
        features_section_label: "Features",
        pricing_section_label: "Pricing",
        contact_section_label: "Contact",
        select_language: "Select language",
        go_to_app: "Go to app",
        new_board: "New board",
        h1_main: "Shared task management.",
        h1_sub: "Simple, with no sign ups.",
        dense_button_label: "Dense",
        dark_button_label: "Dark",
        mobile_button_label: "Mobile",
    },
};
