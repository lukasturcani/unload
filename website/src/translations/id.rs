use super::{Text, Translation};

pub const ID: Translation<&'static str> = Translation {
    id: "id",
    name: "ID - Indonesian",
    text: Text {
        open_main_menu: "Buka menu utama",
        home_section_label: "Beranda",
        features_section_label: "Fitur",
        pricing_section_label: "Harga",
        contact_section_label: "Kontak",
        select_language: "Pilih bahasa",
        go_to_app: "Pergi ke aplikasi",
        app_link: "/id/app",
        home_link: "/id",
        new_board_link: "/id/papan-baru",
        new_board: "Papan baru",
        h1_main: "Manajemen tugas bersama.",
        h1_sub: "Sederhana, tanpa perlu daftar.",
        dense_button_label: "Padat",
        dark_button_label: "Gelap",
        mobile_button_label: "Mobile",
    },
};
