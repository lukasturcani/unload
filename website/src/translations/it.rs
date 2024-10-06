use super::{Text, Translation};

pub const IT: Translation<&'static str> = Translation {
    id: "it",
    name: "IT - Italiano",
    text: Text {
        open_main_menu: "Apri il menu principale",
        home_section_label: "Home",
        features_section_label: "Caratteristiche",
        pricing_section_label: "Prezzi",
        contact_section_label: "Contatti",
        select_language: "Seleziona lingua",
        go_to_app: "Vai all'app",
        new_board: "Nuovo board",
        h1_main: "Gestione condivisa delle attività.",
        h1_sub: "Semplice, senza registrazioni.",
        dense_button_label: "Denso",
        dark_button_label: "Scuro",
        mobile_button_label: "Mobile",
    },
};
