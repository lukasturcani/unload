use super::{Text, Translation};

pub const UK: Translation<&'static str> = Translation {
    id: "uk",
    name: "UK - Українська",
    text: Text {
        open_main_menu: "Відкрити головне меню",
        home_section_label: "Головна",
        features_section_label: "Функції",
        pricing_section_label: "Ціни",
        contact_section_label: "Контакти",
        select_language: "Виберіть мову",
        go_to_app: "Перейти до додатку",
        new_board: "Нова дошка",
        h1_main: "Спільне керування задачами.",
        h1_sub: "Просто, без реєстрації.",
        dense_button_label: "Щільно",
        dark_button_label: "Темна тема",
        mobile_button_label: "Мобільна версія",
    },
};
