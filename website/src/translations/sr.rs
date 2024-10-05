use super::{Text, Translation};

pub const SR: Translation<&'static str> = Translation {
    id: "sr",
    name: "SR - Serbian",
    text: Text {
        open_main_menu: "Otvorite glavni meni",
        home_section_label: "Početna",
        features_section_label: "Funkcionalnosti",
        pricing_section_label: "Cene",
        contact_section_label: "Kontakt",
        select_language: "Izaberite jezik",
        go_to_app: "Idi na aplikaciju",
        app_link: "/sr/aplikacija",
        home_link: "/sr",
        new_board_link: "/sr/novi-tabla",
        new_board: "Nova tabla",
        h1_main: "Zajedničko upravljanje zadacima.",
        h1_sub: "Jednostavno, bez registracije.",
        dense_button_label: "Gusto",
        dark_button_label: "Tamno",
        mobile_button_label: "Mobilna",
    },
};
