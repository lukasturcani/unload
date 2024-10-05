use super::{Text, Translation};

pub const PL: Translation<&'static str> = Translation {
    id: "pl",
    name: "PL - Polski",
    text: Text {
        open_main_menu: "Otwórz główne menu",
        home_section_label: "Strona główna",
        features_section_label: "Funkcje",
        pricing_section_label: "Cennik",
        contact_section_label: "Kontakt",
        select_language: "Wybierz język",
        go_to_app: "Przejdź do aplikacji",
        app_link: "/pl/app",
        home_link: "/pl",
        new_board_link: "/pl/nowa-tablica",
        new_board: "Nowa tablica",
        h1_main: "Wspólne zarządzanie zadaniami.",
        h1_sub: "Proste, bez rejestracji.",
        dense_button_label: "Gęsty",
        dark_button_label: "Ciemny",
        mobile_button_label: "Mobilny",
    },
};
