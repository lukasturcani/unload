use super::{Text, Translation};

pub const DE: Translation<&'static str> = Translation {
    id: "de",
    name: "DE - Deutsch",
    text: Text {
        open_main_menu: "Hauptmenü öffnen",
        home_section_label: "Startseite",
        features_section_label: "Funktionen",
        pricing_section_label: "Preise",
        contact_section_label: "Kontakt",
        select_language: "Sprache wählen",
        go_to_app: "Zur App gehen",
        app_link: "/de/app",
        home_link: "/de",
        new_board_link: "/de/new-board",
        new_board: "Neue Tafel",
        h1_main: "Geteiltes Aufgabenmanagement.",
        h1_sub: "Einfach, ohne Anmeldung.",
        dense_button_label: "Kompakt",
        dark_button_label: "Dunkel",
        mobile_button_label: "Mobil",
    },
};
