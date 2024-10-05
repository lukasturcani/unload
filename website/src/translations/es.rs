use super::{Text, Translation};

pub const ES: Translation<&'static str> = Translation {
    id: "es",
    name: "ES - Español",
    text: Text {
        open_main_menu: "Abrir menú principal",
        home_section_label: "Inicio",
        features_section_label: "Características",
        pricing_section_label: "Precios",
        contact_section_label: "Contacto",
        select_language: "Seleccionar idioma",
        go_to_app: "Ir a la app",
        app_link: "/es/app",
        home_link: "/es",
        new_board_link: "/es/nuevo-tablero",
        new_board: "Nuevo tablero",
        h1_main: "Gestión de tareas compartida.",
        h1_sub: "Simple, sin registros.",
        dense_button_label: "Compacto",
        dark_button_label: "Oscuro",
        mobile_button_label: "Móvil",
    },
};
