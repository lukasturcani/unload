use super::{Text, Translation};

pub const BG: Translation<&'static str> = Translation {
    id: "bg",
    name: "BG - Български",
    text: Text {
        open_main_menu: "Отвори главното меню",
        home_section_label: "Начало",
        features_section_label: "Характеристики",
        pricing_section_label: "Цени",
        contact_section_label: "Контакт",
        select_language: "Избери език",
        go_to_app: "Отиди на приложението",
        app_link: "/bg/app",
        home_link: "/bg",
        new_board_link: "/bg/new-board",
        new_board: "Нова дъска",
        h1_main: "Споделено управление на задачи.",
        h1_sub: "Просто, без регистрации.",
        dense_button_label: "Плътно",
        dark_button_label: "Тъмно",
        mobile_button_label: "Мобилно",
    },
};
