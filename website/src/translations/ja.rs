use super::{Text, Translation};

pub const JA: Translation<&'static str> = Translation {
    id: "ja",
    name: "JA - 日本語",
    text: Text {
        open_main_menu: "メインメニューを開く",
        home_section_label: "ホーム",
        features_section_label: "機能",
        pricing_section_label: "料金",
        contact_section_label: "お問い合わせ",
        select_language: "言語を選択",
        go_to_app: "アプリへ移動",
        app_link: "/ja/app",
        home_link: "/ja",
        new_board_link: "/ja/new-board",
        new_board: "新しいボード",
        h1_main: "共有タスク管理。",
        h1_sub: "簡単、サインアップ不要。",
        dense_button_label: "密集",
        dark_button_label: "ダーク",
        mobile_button_label: "モバイル",
    },
};
