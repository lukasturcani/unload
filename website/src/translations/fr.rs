use super::{Text, Translation};

pub const FR: Translation<&'static str> = Translation {
    id: "fr",
    name: "FR - Français",
    text: Text {
        open_main_menu: "Ouvrir le menu principal",
        home_section_label: "Accueil",
        features_section_label: "Fonctionnalités",
        pricing_section_label: "Tarification",
        contact_section_label: "Contact",
        select_language: "Choisir la langue",
        go_to_app: "Aller à l'application",
        app_link: "/fr/app",
        home_link: "/fr",
        new_board_link: "/fr/nouveau-tableau",
        new_board: "Nouveau tableau",
        h1_main: "Gestion partagée des tâches.",
        h1_sub: "Simple, sans inscriptions.",
        dense_button_label: "Dense",
        dark_button_label: "Sombre",
        mobile_button_label: "Mobile",
    },
};
