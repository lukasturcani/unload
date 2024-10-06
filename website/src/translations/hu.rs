use super::{Text, Translation};

pub const HU: Translation<&'static str> = Translation {
    id: "hu",
    name: "HU - Magyar",
    text: Text {
        open_main_menu: "Főmenü megnyitása",
        home_section_label: "Főoldal",
        features_section_label: "Funkciók",
        pricing_section_label: "Árazás",
        contact_section_label: "Kapcsolat",
        select_language: "Nyelv kiválasztása",
        go_to_app: "Alkalmazás megnyitása",
        new_board: "Új tábla",
        h1_main: "Megosztott feladatkezelés.",
        h1_sub: "Egyszerű, regisztráció nélkül.",
        dense_button_label: "Sűrű",
        dark_button_label: "Sötét",
        mobile_button_label: "Mobil",
    },
};
