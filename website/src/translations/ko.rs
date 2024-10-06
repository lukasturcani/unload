use super::{Text, Translation};

pub const KO: Translation<&'static str> = Translation {
    id: "ko",
    name: "KO - 한국어",
    text: Text {
        open_main_menu: "메인 메뉴 열기",
        home_section_label: "홈",
        features_section_label: "기능",
        pricing_section_label: "가격",
        contact_section_label: "연락처",
        select_language: "언어 선택",
        go_to_app: "앱으로 이동",
        new_board: "새 보드",
        h1_main: "공유 작업 관리.",
        h1_sub: "간단하고, 가입 불필요.",
        dense_button_label: "밀집",
        dark_button_label: "어두운",
        mobile_button_label: "모바일",
    },
};
