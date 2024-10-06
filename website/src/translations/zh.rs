use super::{Text, Translation};

pub const ZH: Translation<&'static str> = Translation {
    id: "zh",
    name: "ZH - 中文",
    text: Text {
        open_main_menu: "打开主菜单",
        home_section_label: "首页",
        features_section_label: "功能",
        pricing_section_label: "价格",
        contact_section_label: "联系",
        select_language: "选择语言",
        go_to_app: "进入应用",
        new_board: "新建看板",
        h1_main: "共享任务管理。",
        h1_sub: "简单，无需注册。",
        dense_button_label: "密集",
        dark_button_label: "暗色",
        mobile_button_label: "移动版",
    },
};
