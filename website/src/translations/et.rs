use super::{Text, Translation};

pub const ET: Translation<&'static str> = Translation {
    id: "et",
    name: "ET - Estonian",
    text: Text {
        open_main_menu: "Ava põhimenüü",
        home_section_label: "Avaleht",
        features_section_label: "Funktsioonid",
        pricing_section_label: "Hinnakiri",
        contact_section_label: "Kontakt",
        select_language: "Vali keel",
        go_to_app: "Mine rakendusse",
        app_link: "/et/app",
        home_link: "/et",
        new_board_link: "/et/new-board",
        new_board: "Uus tahvel",
        h1_main: "Jagatud ülesannete haldamine.",
        h1_sub: "Lihtne, ilma registreerimiseta.",
        dense_button_label: "Tihe",
        dark_button_label: "Tume",
        mobile_button_label: "Mobiilne",
    },
};
