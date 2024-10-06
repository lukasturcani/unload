use super::{Text, Translation};

pub const HI: Translation<&'static str> = Translation {
    id: "hi",
    name: "HI - हि\u{902}दी",
    text: Text {
        open_main_menu: "म\u{941}ख\u{94d}य म\u{947}न\u{942} खोल\u{947}\u{902}",
        home_section_label: "होम",
        features_section_label: "विश\u{947}षताए\u{901}",
        pricing_section_label: "म\u{942}ल\u{94d}य निर\u{94d}धारण",
        contact_section_label: "स\u{902}पर\u{94d}क कर\u{947}\u{902}",
        select_language: "भाषा च\u{941}न\u{947}\u{902}",
        go_to_app: "ऐप पर जाए\u{902}",
        new_board: "नया बोर\u{94d}ड",
        h1_main: "साझा कार\u{94d}य प\u{94d}रब\u{902}धन।",
        h1_sub: "सरल, बिना साइन अप किए।",
        dense_button_label: "घना",
        dark_button_label: "डार\u{94d}क",
        mobile_button_label: "मोबाइल",
    },
};
