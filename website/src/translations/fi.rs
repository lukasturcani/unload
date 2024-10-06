use super::{Text, Translation};

pub const FI: Translation<&'static str> = Translation {
    id: "fi",
    name: "FI - Finnish",
    text: Text {
        open_main_menu: "Avaa päävalikko",
        home_section_label: "Etusivu",
        features_section_label: "Ominaisuudet",
        pricing_section_label: "Hinnoittelu",
        contact_section_label: "Yhteystiedot",
        select_language: "Valitse kieli",
        go_to_app: "Siirry sovellukseen",
        new_board: "Uusi taulu",
        h1_main: "Jaettu tehtävien hallinta.",
        h1_sub: "Yksinkertainen, eikä vaadi rekisteröitymistä.",
        dense_button_label: "Tiivis",
        dark_button_label: "Tumma",
        mobile_button_label: "Mobiili",
    },
};
