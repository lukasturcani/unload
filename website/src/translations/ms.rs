use super::{Text, Translation};

pub const MS: Translation<&'static str> = Translation {
    id: "ms",
    name: "MS - Malay",
    text: Text {
        open_main_menu: "Buka menu utama",
        home_section_label: "Laman Utama",
        features_section_label: "Fungsi",
        pricing_section_label: "Harga",
        contact_section_label: "Hubungi",
        select_language: "Pilih bahasa",
        go_to_app: "Pergi ke aplikasi",
        app_link: "/ms/app",
        home_link: "/ms",
        new_board_link: "/ms/new-board",
        new_board: "Papan baru",
        h1_main: "Pengurusan tugas bersama.",
        h1_sub: "Mudah, tanpa mendaftar.",
        dense_button_label: "Padat",
        dark_button_label: "Gelap",
        mobile_button_label: "Mudah alih",
    },
};
