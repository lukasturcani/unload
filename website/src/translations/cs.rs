use super::{Text, Translation};

pub const CS: Translation<&'static str> = Translation {
    id: "cs",
    name: "CS - Čeština",
    text: Text {
        open_main_menu: "Otevřít hlavní menu",
        home_section_label: "Domů",
        features_section_label: "Funkce",
        pricing_section_label: "Ceník",
        contact_section_label: "Kontakt",
        select_language: "Vybrat jazyk",
        go_to_app: "Přejít do aplikace",
        new_board: "Nová deska",
        h1_main: "Sdílená správa úkolů.",
        h1_sub: "Jednoduché, bez registrace.",
        dense_button_label: "Husté",
        dark_button_label: "Tmavý",
        mobile_button_label: "Mobilní",
    },
};
