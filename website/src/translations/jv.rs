use super::{Text, Translation};

pub const JV: Translation<&'static str> = Translation {
    id: "jv",
    name: "JV - Javanese",
    text: Text {
        open_main_menu: "Buka menu utama",
        home_section_label: "Beranda",
        features_section_label: "Fitur",
        pricing_section_label: "Harga",
        contact_section_label: "Kontak",
        select_language: "Pilih basa",
        go_to_app: "Pergi ke aplikasi",
        app_link: "/jv/app",
        home_link: "/jv",
        new_board_link: "/jv/new-board",
        new_board: "Papan anyar",
        h1_main: "Manajemen tugas bersama.",
        h1_sub: "Sederhana, tanpa perlu daftar.",
        dense_button_label: "Padat",
        dark_button_label: "Peteng",
        mobile_button_label: "Seluler",
    },
};
