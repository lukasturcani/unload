use super::{Text, Translation};

pub const KA: Translation<&'static str> = Translation {
    id: "ka",
    name: "KA - ქართული",
    text: Text {
        open_main_menu: "მთავარი მენიუს გაღება",
        home_section_label: "მთავარი",
        features_section_label: "ფუნქციები",
        pricing_section_label: "ფასები",
        contact_section_label: "კონტაქტი",
        select_language: "ენის არჩევა",
        go_to_app: "აპლიკაციაში შესვლა",
        new_board: "ახალი დაფა",
        h1_main: "გააზიარებული დავალებების მართვა.",
        h1_sub: "მარტივი, რეგისტრაციის გარეშე.",
        dense_button_label: "ჭედვური",
        dark_button_label: "მუქი",
        mobile_button_label: "მობილური",
    },
};
