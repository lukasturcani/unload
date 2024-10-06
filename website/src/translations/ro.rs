use super::{Text, Translation};

pub const RO: Translation<&'static str> = Translation {
    id: "ro",
    name: "RO - Română",
    text: Text {
        open_main_menu: "Deschide meniul principal",
        home_section_label: "Acasă",
        features_section_label: "Caracteristici",
        pricing_section_label: "Prețuri",
        contact_section_label: "Contact",
        select_language: "Selectează limba",
        go_to_app: "Accesează aplicația",
        new_board: "Tablou nou",
        h1_main: "Gestiunea sarcinilor partajate.",
        h1_sub: "Simplu, fără înregistrări.",
        dense_button_label: "Dens",
        dark_button_label: "Întunecat",
        mobile_button_label: "Mobil",
    },
};
