use super::{Text, Translation};

pub const PT: Translation<&'static str> = Translation {
    id: "pt",
    name: "PT - Português",
    text: Text {
        open_main_menu: "Abrir menu principal",
        home_section_label: "Início",
        features_section_label: "Recursos",
        pricing_section_label: "Preços",
        contact_section_label: "Contato",
        select_language: "Selecionar linguagem",
        go_to_app: "Ir para o app",
        app_link: "/pt/app",
        home_link: "/pt",
        new_board_link: "/pt/novo-quadro",
        new_board: "Novo quadro",
        h1_main: "Gerenciamento de tarefas compartilhadas.",
        h1_sub: "Simples, sem inscrições.",
        dense_button_label: "Denso",
        dark_button_label: "Escuro",
        mobile_button_label: "Móvel",
    },
};
