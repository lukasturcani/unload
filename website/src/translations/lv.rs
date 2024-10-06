use super::{Text, Translation};

pub const LV: Translation<&'static str> = Translation {
    id: "lv",
    name: "LV - Latviešu",
    text: Text {
        open_main_menu: "Atvērt galveno izvēlni",
        home_section_label: "Sākumlapa",
        features_section_label: "Funkcijas",
        pricing_section_label: "Cenas",
        contact_section_label: "Kontakti",
        select_language: "Izvēlēties valodu",
        go_to_app: "Doties uz lietotni",
        new_board: "Jauna tabula",
        h1_main: "Koplietojamo uzdevumu pārvaldība.",
        h1_sub: "Vienkārši, bez reģistrēšanās.",
        dense_button_label: "Blīvs",
        dark_button_label: "Tumšs",
        mobile_button_label: "Mobilais",
    },
};
