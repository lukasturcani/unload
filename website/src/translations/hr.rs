use super::{Text, Translation};

pub const HR: Translation<&'static str> = Translation {
    id: "hr",
    name: "HR - Hrvatski",
    text: Text {
        open_main_menu: "Otvori glavni izbornik",
        home_section_label: "Početna",
        features_section_label: "Značajke",
        pricing_section_label: "Cijene",
        contact_section_label: "Kontakt",
        select_language: "Odaberi jezik",
        go_to_app: "Idi na aplikaciju",
        new_board: "Nova ploča",
        h1_main: "Zajedničko upravljanje zadacima.",
        h1_sub: "Jednostavno, bez registracije.",
        dense_button_label: "Gusto",
        dark_button_label: "Tamno",
        mobile_button_label: "Mobilno",
    },
};
