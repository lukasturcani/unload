use super::{Text, Translation};

pub const TL: Translation<&'static str> = Translation {
    id: "tl",
    name: "TL - Tagalog",
    text: Text {
        open_main_menu: "Buksan ang pangunahing menu",
        home_section_label: "Home",
        features_section_label: "Mga Tampok",
        pricing_section_label: "Presyo",
        contact_section_label: "Kontak",
        select_language: "Piliin ang wika",
        go_to_app: "Pumunta sa app",
        new_board: "Bagong board",
        h1_main: "Pinagsamang pamamahala ng gawain.",
        h1_sub: "Simple, walang rehistro.",
        dense_button_label: "Siksik",
        dark_button_label: "Madilim",
        mobile_button_label: "Mobile",
    },
};
