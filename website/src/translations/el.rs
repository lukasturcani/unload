use super::{Text, Translation};

pub const EL: Translation<&'static str> = Translation {
    id: "el",
    name: "EL - Ελληνικά",
    text: Text {
        open_main_menu: "Άνοιγμα κύριου μενού",
        home_section_label: "Αρχική",
        features_section_label: "Χαρακτηριστικά",
        pricing_section_label: "Τιμολογιακή Πολιτική",
        contact_section_label: "Επικοινωνία",
        select_language: "Επιλογή γλώσσας",
        go_to_app: "Μετάβαση στην εφαρμογή",
        app_link: "/el/app",
        home_link: "/el",
        new_board_link: "/el/new-board",
        new_board: "Νέος πίνακας",
        h1_main: "Διαχείριση εργασιών σε κοινή χρήση.",
        h1_sub: "Απλό, χωρίς εγγραφές.",
        dense_button_label: "Πυκνό",
        dark_button_label: "Σκούρο",
        mobile_button_label: "Κινητό",
    },
};
