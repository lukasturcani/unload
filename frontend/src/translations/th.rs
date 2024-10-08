use super::{Text, Translation};

pub const TH: Translation<&'static str> = Translation {
    id: "th",
    name: "TH - ภาษาไทย",
    text: Text {
        to_do_column_title: "งานท\u{e35}\u{e48}ต\u{e49}องทำ",
        in_progress_column_title: "กำล\u{e31}งดำเน\u{e34}นการ",
        done_column_title: "เสร\u{e47}จส\u{e34}\u{e49}น",
        pick_language_tooltip: "เล\u{e37}อกภาษา",
        toggle_show_themes_tooltip: "เปล\u{e35}\u{e48}ยนธ\u{e35}ม",
        toggle_dense_view_tooltip: "สล\u{e31}บม\u{e38}มมองแน\u{e48}น",
        edit_board_title_tooltip: "แก\u{e49}ไขช\u{e37}\u{e48}อ",
        board_title_input_label: "ช\u{e37}\u{e48}อ",
        board_title_update_form_label: "อ\u{e31}ปเดตช\u{e37}\u{e48}อบอร\u{e4c}ด",
        set_board_title_button_label: "ต\u{e31}\u{e49}งช\u{e37}\u{e48}อ",
        cancel_board_title_update_button_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตช\u{e37}\u{e48}อ",
        task_title_input_label: "ช\u{e37}\u{e48}อ",
        edit_task_title_tooltip: "แก\u{e49}ไขช\u{e37}\u{e48}อ",
        task_title_update_form_label: "อ\u{e31}ปเดตช\u{e37}\u{e48}องาน",
        set_task_title_button_label: "ต\u{e31}\u{e49}งช\u{e37}\u{e48}อ",
        cancel_task_title_update_button_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตช\u{e37}\u{e48}อ",
        set_task_status_section_label: "ต\u{e31}\u{e49}งสถานะงาน",
        to_do_button_tooltip: "งานท\u{e35}\u{e48}ต\u{e49}องทำ",
        in_progress_button_tooltip: "กำล\u{e31}งดำเน\u{e34}นการ",
        done_button_tooltip: "เสร\u{e47}จส\u{e34}\u{e49}น",
        task_actions_section_label: "การกระทำของงาน",
        duplicate_task_button_tooltip: "จำลองงาน",
        archive_task_button_tooltip: "เก\u{e47}บถาวรงาน",
        unarchive_task_button_tooltip: "ก\u{e39}\u{e49}ค\u{e37}นงาน",
        assignees_section_label: "ผ\u{e39}\u{e49}ร\u{e31}บผ\u{e34}ดชอบ",
        assign_user_toggle_button_tooltip: "กำหนดผ\u{e39}\u{e49}ใช\u{e49}",
        toggle_user_filter_button_label: "สล\u{e31}บการกรองผ\u{e39}\u{e49}ใช\u{e49}",
        assignee_selection_section_label: "เล\u{e37}อกผ\u{e39}\u{e49}ร\u{e31}บผ\u{e34}ดชอบ",
        add_user_button_label: "เพ\u{e34}\u{e48}มผ\u{e39}\u{e49}ใช\u{e49}",
        add_user_form_label: "เพ\u{e34}\u{e48}มผ\u{e39}\u{e49}ใช\u{e49}",
        user_name_input_label: "ช\u{e37}\u{e48}อ",
        cancel_adding_new_user_button_label: "ยกเล\u{e34}กการเพ\u{e34}\u{e48}มผ\u{e39}\u{e49}ใช\u{e49}",
        remove_user_from_task_button_label: "ลบผ\u{e39}\u{e49}ใช\u{e49}ออกจากงาน",
        tags_section_label: "แท\u{e47}ก",
        tag_selection_section_label: "เล\u{e37}อกแท\u{e47}ก",
        add_tag_button_label: "เพ\u{e34}\u{e48}มแท\u{e47}ก",
        add_tag_form_label: "เพ\u{e34}\u{e48}มแท\u{e47}ก",
        tag_name_input_label: "ช\u{e37}\u{e48}อ",
        add_tag_toggle_button_tooltip: "เพ\u{e34}\u{e48}มแท\u{e47}ก",
        cancel_adding_new_tag_button_label: "ยกเล\u{e34}กการเพ\u{e34}\u{e48}มแท\u{e47}ก",
        toggle_tag_filter_button_label: "สล\u{e31}บการกรองแท\u{e47}ก",
        remove_tag_from_task_button_label: "ลบแท\u{e47}กจากงาน",
        toggle_expand_task_button_label: "สล\u{e31}บขยายงาน",
        due_date_section_label: "กำหนดว\u{e31}น",
        edit_due_date_tooltip: "แก\u{e49}ไขว\u{e31}นกำหนด",
        due_date_form_label: "ต\u{e31}\u{e49}งว\u{e31}นกำหนด",
        due_date_input_label: "กำหนด",
        set_due_date_button_label: "ต\u{e31}\u{e49}งว\u{e31}นกำหนด",
        cancel_due_date_update_button_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตว\u{e31}นกำหนด",
        color_picker_legend_label: "ส\u{e35}",
        description_update_form_label: "อ\u{e31}ปเดตคำอธ\u{e34}บาย",
        set_description_button_label: "ต\u{e31}\u{e49}งคำอธ\u{e34}บาย",
        cancel_description_update_button_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตคำอธ\u{e34}บาย",
        bullet_points_button_tooltip: "จ\u{e38}ดแสดงรายการ",
        task_list_button_tooltip: "รายการงาน",
        description_text_area_label: "คำอธ\u{e34}บาย",
        description_section_label: "คำอธ\u{e34}บาย",
        edit_description_tooltip: "แก\u{e49}ไขคำอธ\u{e34}บาย",
        additional_actions_section_label: "การกระทำเพ\u{e34}\u{e48}มเต\u{e34}ม",
        delete_task_tooltip: "ลบงาน",
        edit_tag_color_form_label: "แก\u{e49}ไขส\u{e35}",
        edit_tag_color_button_label: "แก\u{e49}ไขส\u{e35}",
        set_tag_color_button_label: "ต\u{e31}\u{e49}งส\u{e35}",
        cancel_tag_color_update_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตส\u{e35}",
        edit_tag_name_button_label: "แก\u{e49}ไขช\u{e37}\u{e48}อ",
        edit_tag_name_form_label: "แก\u{e49}ไขช\u{e37}\u{e48}อ",
        set_tag_name_button_label: "ต\u{e31}\u{e49}งช\u{e37}\u{e48}อ",
        cancel_tag_name_update_button_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตช\u{e37}\u{e48}อ",
        delete_tag_button_label: "ลบแท\u{e47}ก",
        archive_tag_button_label: "เก\u{e47}บถาวรแท\u{e47}ก",
        unarchive_tag_button_label: "ก\u{e39}\u{e49}ค\u{e37}นแท\u{e47}ก",
        edit_user_color_form_label: "แก\u{e49}ไขส\u{e35}",
        set_user_color_button_label: "ต\u{e31}\u{e49}งส\u{e35}",
        cancel_user_color_update_button_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตส\u{e35}",
        edit_user_color_button_label: "แก\u{e49}ไขส\u{e35}",
        edit_user_name_form_label: "แก\u{e49}ไขช\u{e37}\u{e48}อ",
        set_user_name_button_label: "ต\u{e31}\u{e49}งช\u{e37}\u{e48}อ",
        cancel_user_name_update_button_label: "ยกเล\u{e34}กการอ\u{e31}ปเดตช\u{e37}\u{e48}อ",
        edit_user_name_button_label: "แก\u{e49}ไขช\u{e37}\u{e48}อ",
        delete_user_button_label: "ลบผ\u{e39}\u{e49}ใช\u{e49}",
        task_status_section_label: "สถานะงาน",
        filters_section_label: "ต\u{e31}วกรอง",
        languages_section_title: "ภาษา",
        custom_task_button_label: "งานท\u{e35}\u{e48}กำหนดเอง",
        board_list_section_label: "รายการบอร\u{e4c}ด",
        join_board_button_label: "เข\u{e49}าร\u{e48}วมบอร\u{e4c}ด",
        create_new_board_button_label: "สร\u{e49}างบอร\u{e4c}ดใหม\u{e48}",
        or_label: "หร\u{e37}อ",
        chat_gpt_limit_exceeded_title: "จำก\u{e31}ดการใช\u{e49}งาน ChatGPT เก\u{e34}น",
        chat_gpt_limit_exceeded_content: "ค\u{e38}ณได\u{e49}เก\u{e34}นข\u{e35}ดจำก\u{e31}ดการเร\u{e35}ยกใช\u{e49} ChatGPT กร\u{e38}ณาลองใหม\u{e48}อ\u{e35}กคร\u{e31}\u{e49}งในภายหล\u{e31}ง",
        chat_gpt_waiting_message: "กำล\u{e31}งพ\u{e39}ดค\u{e38}ยก\u{e31}บ ChatGPT...",
        chat_gpt_error_title: "ข\u{e49}อผ\u{e34}ดพลาดของ ChatGPT",
        chat_gpt_error_content: "เก\u{e34}ดข\u{e49}อผ\u{e34}ดพลาดขณะเช\u{e37}\u{e48}อมต\u{e48}อก\u{e31}บ ChatGPT กร\u{e38}ณาลองใหม\u{e48}อ\u{e35}กคร\u{e31}\u{e49}งในภายหล\u{e31}ง",
        chat_gpt_prompt_input_title: "คำส\u{e31}\u{e48}ง ChatGPT",
        chat_gpt_daily_attempts_left: "พยายามรายว\u{e31}นคงเหล\u{e37}อ",
        chat_gpt_prompt_input_content: "หร\u{e37}อเล\u{e37}อกหน\u{e36}\u{e48}งจากคำแนะนำด\u{e49}านล\u{e48}าง:",
        chat_gpt_prompt_input_form_label: "คำส\u{e31}\u{e48}ง ChatGPT",
        chat_gpt_prompt_input_label: "คำส\u{e31}\u{e48}ง:",
        suggest_cupcake_recipe_prompt: "แนะนำส\u{e39}ตรเค\u{e49}กค\u{e31}พเค\u{e49}ก",
        paint_bedroom_prompt: "ทาส\u{e35}ห\u{e49}องนอน",
        friends_over_for_bbq_prompt: "เช\u{e34}ญเพ\u{e37}\u{e48}อนมางาน BBQ",
        prepare_for_rome_vacation_prompt: "เตร\u{e35}ยมต\u{e31}วสำหร\u{e31}บการพ\u{e31}กผ\u{e48}อนท\u{e35}\u{e48}โรม",
        house_tidy_prompt: "จ\u{e31}ดบ\u{e49}านให\u{e49}เร\u{e35}ยบร\u{e49}อย",
        fix_fence_prompt: "ซ\u{e48}อมแซมร\u{e31}\u{e49}ว",
        join_board_form_label: "เข\u{e49}าร\u{e48}วมบอร\u{e4c}ด",
        join_board_input_label: "ช\u{e37}\u{e48}อบอร\u{e4c}ด",
        cancel_joining_board_button_label: "ยกเล\u{e34}กการเข\u{e49}าร\u{e48}วมบอร\u{e4c}ด",
        add_task_button_label: "เพ\u{e34}\u{e48}มงาน",
        remove_board_button_label: "ลบบอร\u{e4c}ด",
        new_task_form_label: "งานใหม\u{e48}",
        cancel_adding_new_task_button_label: "ยกเล\u{e34}กการเพ\u{e34}\u{e48}มงานใหม\u{e48}",
        navigation_section_label: "การนำทาง",
        toggle_actions_drawer_button_label: "สล\u{e31}บล\u{e34}\u{e49}นช\u{e31}กการกระทำ",
        toggle_show_filters_button_label: "สล\u{e31}บการแสดงต\u{e31}วกรอง",
        themes_section_label: "ธ\u{e35}ม",
        close_theme_selector_button_label: "ป\u{e34}ดต\u{e31}วเล\u{e37}อกธ\u{e35}ม",
        close_filters_button_label: "ป\u{e34}ดต\u{e31}วกรอง",
        board_link: "บอร\u{e4c}ด",
        tags_link: "แท\u{e47}ก",
        users_link: "ผ\u{e39}\u{e49}ใช\u{e49}",
        archive_link: "เก\u{e47}บถาวร",
    },
};
