use super::{Text, Translation};

pub const AR: Translation<&'static str> = Translation {
    id: "ar",
    name: "AR - العربية",
    text: Text {
        open_main_menu: "افتح القائمة الرئيسية",
        home_section_label: "الصفحة الرئيسية",
        features_section_label: "الميزات",
        pricing_section_label: "التسعير",
        contact_section_label: "اتصل بنا",
        select_language: "اختر اللغة",
        go_to_app: "اذهب إلى التطبيق",
        new_board: "لوحة جديدة",
        h1_main: "إدارة المهام المشتركة.",
        h1_sub: "بسيط، بدون تسجيل.",
        dense_button_label: "كثيف",
        dark_button_label: "داكن",
        mobile_button_label: "موبايل",
    },
};
