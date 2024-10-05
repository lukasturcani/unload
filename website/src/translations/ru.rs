use super::{Text, Translation};

pub const RU: Translation<&'static str> = Translation {
    id: "ru",
    name: "RU - Русский",
    text: Text {
        open_main_menu: "Открыть главное меню",
        home_section_label: "Главная",
        features_section_label: "Функции",
        pricing_section_label: "Цены",
        contact_section_label: "Контакты",
        select_language: "Выбрать язык",
        go_to_app: "Перейти к приложению",
        app_link: "/ru/app",
        home_link: "/ru",
        new_board_link: "/ru/new-board",
        new_board: "Новая доска",
        h1_main: "Совместное управление задачами.",
        h1_sub: "Просто, без регистрации.",
        dense_button_label: "Компактный",
        dark_button_label: "Темный",
        mobile_button_label: "Мобильный",
    },
};
