use super::{Text, Translation};

pub const NL: Translation<&'static str> = Translation {
    id: "nl",
    name: "NL - Nederlands",
    text: Text {
        open_main_menu: "Hoofdmenu openen",
        home_section_label: "Home",
        features_section_label: "Functionaliteiten",
        pricing_section_label: "Prijzen",
        contact_section_label: "Contact",
        select_language: "Selecteer taal",
        go_to_app: "Ga naar app",
        app_link: "/nl/app",
        home_link: "/nl",
        new_board_link: "/nl/nieuw-bord",
        new_board: "Nieuw bord",
        h1_main: "Gedeeld takenbeheer.",
        h1_sub: "Eenvoudig, zonder aanmelden.",
        dense_button_label: "Compact",
        dark_button_label: "Donker",
        mobile_button_label: "Mobiel",
    },
};
