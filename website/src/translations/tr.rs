use super::{Text, Translation};

pub const TR: Translation<&'static str> = Translation {
    id: "tr",
    name: "TR - Türkçe",
    text: Text {
        open_main_menu: "Ana menüyü aç",
        home_section_label: "Anasayfa",
        features_section_label: "Özellikler",
        pricing_section_label: "Fiyatlandırma",
        contact_section_label: "İletişim",
        select_language: "Dil seç",
        go_to_app: "Uygulamaya git",
        app_link: "/tr/app",
        home_link: "/tr",
        new_board_link: "/tr/yeni-tahta",
        new_board: "Yeni tahta",
        h1_main: "Paylaşılan görev yönetimi.",
        h1_sub: "Basit, üye olmadan.",
        dense_button_label: "Yoğun",
        dark_button_label: "Koyu",
        mobile_button_label: "Mobil",
    },
};
