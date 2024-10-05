use super::{Text, Translation};

pub const FA: Translation<&'static str> = Translation {
    id: "fa",
    name: "FA - فارسی",
    text: Text {
        open_main_menu: "باز کردن منوی اصلی",
        home_section_label: "خانه",
        features_section_label: "ویژگی\u{200c}ها",
        pricing_section_label: "قیمت\u{200c}ها",
        contact_section_label: "تماس",
        select_language: "انتخاب زبان",
        go_to_app: "رفتن به برنامه",
        app_link: "/fa/app",
        home_link: "/fa",
        new_board_link: "/fa/new-board",
        new_board: "بورد جدید",
        h1_main: "مدیریت وظایف مشترک.",
        h1_sub: "ساده، بدون نیاز به ثبت\u{200c}نام.",
        dense_button_label: "فشرده",
        dark_button_label: "تیره",
        mobile_button_label: "موبایل",
    },
};
