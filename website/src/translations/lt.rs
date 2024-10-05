use super::{Text, Translation};

pub const LT: Translation<&'static str> = Translation {
    id: "lt",
    name: "LT - Lietuvių",
    text: Text {
        open_main_menu: "Atidaryti pagrindinį meniu",
        home_section_label: "Pagrindinis",
        features_section_label: "Funkcijos",
        pricing_section_label: "Kainos",
        contact_section_label: "Kontaktai",
        select_language: "Pasirinkti kalbą",
        go_to_app: "Eiti į programą",
        app_link: "/lt/app",
        home_link: "/lt",
        new_board_link: "/lt/nauja-lenta",
        new_board: "Nauja lenta",
        h1_main: "Bendras užduočių valdymas.",
        h1_sub: "Paprasta, be registracijų.",
        dense_button_label: "Tankus",
        dark_button_label: "Tamsus",
        mobile_button_label: "Mobilus",
    },
};
