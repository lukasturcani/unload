use super::{Text, Translation};

pub const IG: Translation<&'static str> = Translation {
    id: "ig",
    name: "IG - Igbo",
    text: Text {
        to_do_column_title: "Iru Eme",
        in_progress_column_title: "Na-arụ Ọrụ",
        done_column_title: "Emeela",
        pick_language_tooltip: "Họrọ Asụsụ",
        toggle_show_themes_tooltip: "Gbanwee Atụmatụ",
        toggle_dense_view_tooltip: "Gbanwee Elu Elu",
        edit_board_title_tooltip: "Mezi Akụkọ Ndị",
        board_title_input_label: "Aha",
        board_title_update_form_label: "Melite Akụkọ Ndị",
        set_board_title_button_label: "Tọọ Aha",
        cancel_board_title_update_button_label: "Kagbuo Imelite Aha",
        task_title_input_label: "Aha",
        edit_task_title_tooltip: "Mezie Aha",
        task_title_update_form_label: "Melite Aha Ọrụ",
        set_task_title_button_label: "Tọọ Aha",
        cancel_task_title_update_button_label: "Kagbuo Imelite Aha",
        set_task_status_section_label: "Tọọ Ọnọdụ Ọrụ",
        to_do_button_tooltip: "Iru Eme",
        in_progress_button_tooltip: "Na-arụ Ọrụ",
        done_button_tooltip: "Emeela",
        task_actions_section_label: "Omume Ọrụ",
        duplicate_task_button_tooltip: "Mepụta Ọrụ",
        archive_task_button_tooltip: "Chekwa Ọrụ",
        unarchive_task_button_tooltip: "Weghachite Ọrụ",
        assignees_section_label: "Ndị Edere Ọrụ",
        assign_user_toggle_button_tooltip: "Kpezie Onye Ọrụ",
        toggle_user_filter_button_label: "gbanwee nzacha onye ọrụ",
        assignee_selection_section_label: "nhọrọ onye ọrụ",
        add_user_button_label: "Tinye Onye Ọrụ",
        add_user_form_label: "Tinye Onye Ọrụ",
        user_name_input_label: "Aha",
        cancel_adding_new_user_button_label: "kagbuo ịtinye onye ọrụ",
        remove_user_from_task_button_label: "wepụ onye ọrụ na ọrụ",
        tags_section_label: "Mkpado",
        tag_selection_section_label: "Nhọrọ Mkpado",
        add_tag_button_label: "Tinye Mkpado",
        add_tag_form_label: "Tinye Mkpado",
        tag_name_input_label: "Aha",
        add_tag_toggle_button_tooltip: "Tinye Mkpado",
        cancel_adding_new_tag_button_label: "kagbuo ịtinye mkpado",
        toggle_tag_filter_button_label: "gbanwee nzacha mkpado",
        remove_tag_from_task_button_label: "wepụ mkpado na ọrụ",
        toggle_expand_task_button_label: "gbanwee imepụta ọrụ",
        due_date_section_label: "ụbọchị ọchịchị",
        edit_due_date_tooltip: "Mezi Ụbọchị Ọchịchị",
        due_date_form_label: "tọpụta ụbọchị ọchịchị",
        due_date_input_label: "Ụbọchị",
        set_due_date_button_label: "tọpụta ụbọchị",
        cancel_due_date_update_button_label: "kagbuo ịtọputa ụbọchị",
        color_picker_legend_label: "Agba",
        description_update_form_label: "melite nkọwa",
        set_description_button_label: "tọpụta nkọwa",
        cancel_description_update_button_label: "kagbuo imelite nkọwa",
        bullet_points_button_tooltip: "Manya ihe",
        task_list_button_tooltip: "Ndepụta Ọrụ",
        description_text_area_label: "Nkọwa",
        description_section_label: "Nkọwa",
        edit_description_tooltip: "Mezie Nkọwa",
        additional_actions_section_label: "omume ndị ọzọ",
        delete_task_tooltip: "Hichapụ Ọrụ",
        edit_tag_color_form_label: "Mezi Agba",
        edit_tag_color_button_label: "Mezi Agba",
        set_tag_color_button_label: "Tọpụta Agba",
        cancel_tag_color_update_label: "Kagbuo imelite agba",
        edit_tag_name_button_label: "Mezi Aha",
        edit_tag_name_form_label: "Mezi Aha",
        set_tag_name_button_label: "Tọpụta Aha",
        cancel_tag_name_update_button_label: "Kagbuo imelite aha",
        delete_tag_button_label: "Hichapụ Mkpado",
        archive_tag_button_label: "Chekwa Mkpado",
        unarchive_tag_button_label: "Weghachite Mkpado",
        edit_user_color_form_label: "Mezi Agba",
        set_user_color_button_label: "Tọpụta Agba",
        cancel_user_color_update_button_label: "Kagbuo imelite agba",
        edit_user_color_button_label: "Mezi Agba",
        edit_user_name_form_label: "Mezi Aha",
        set_user_name_button_label: "Tọpụta Aha",
        cancel_user_name_update_button_label: "Kagbuo imelite aha",
        edit_user_name_button_label: "Mezi Aha",
        delete_user_button_label: "Hichapụ Onye Ọrụ",
        task_status_section_label: "Ọnọdụ Ọrụ",
        filters_section_label: "Nzacha",
        languages_section_title: "Asụsụ",
        custom_task_button_label: "Ọrụ ahaziri achazi",
        board_list_section_label: "Ndepụta Akụkọ Ndị",
        join_board_button_label: "Soro Akụkọ Ndị",
        create_new_board_button_label: "Mepụta Akụkọ Ndị Ọhụrụ",
        or_label: "ma ọ bụ",
        chat_gpt_limit_exceeded_title: "Ọnụọgụ ChatGPT eruola ókè",
        chat_gpt_limit_exceeded_content: "Ị eruola ókè nke oku ChatGPT. Biko nwalee ọzọ.",
        chat_gpt_waiting_message: "Na-ekwurita ChatGPT...",
        chat_gpt_error_title: "ChatGPT Ndudue",
        chat_gpt_error_content: "Nwee ndudue mgbe ana eme ihe iji jikọọ ChatGPT. Biko nwalee ọzọ.",
        chat_gpt_prompt_input_title: "ChatGPT Ajụjụ",
        chat_gpt_daily_attempts_left: "ọnwa kwa ụbọchị fọdụrụ",
        chat_gpt_prompt_input_content: "ma ọ bụ họrọ otu n'ime ọkpụsịrị nke dị n'okpuru:",
        chat_gpt_prompt_input_form_label: "chat gpt ajụjụ",
        chat_gpt_prompt_input_label: "Ajụjụ:",
        suggest_cupcake_recipe_prompt: "gosi nkuzi cupcake",
        paint_bedroom_prompt: "agba ụra n'ulo anyị",
        friends_over_for_bbq_prompt: "enyi biara n'ulo maka BBQ",
        prepare_for_rome_vacation_prompt: "kwadebe maka ezumike na Rome",
        house_tidy_prompt: "ndokwa ụlọ",
        fix_fence_prompt: "dozie mgbidi",
        join_board_form_label: "Soro Akụkọ Ndị",
        join_board_input_label: "Aha Akụkọ Ndị",
        cancel_joining_board_button_label: "Kagbuo Iso Akụkọ Ndị",
        add_task_button_label: "Tinye Ọrụ",
        remove_board_button_label: "Wepụ Akụkọ Ndị",
        new_task_form_label: "Ọrụ Ọhụrụ",
        cancel_adding_new_task_button_label: "Kagbuo Ịtinye Ọrụ Ọhụrụ",
        navigation_section_label: "Ntuziaka",
        toggle_actions_drawer_button_label: "Gbaa Drawer Omume",
        toggle_show_filters_button_label: "Chanwe iHụ Nzacha",
        themes_section_label: "Atụmatụ",
        close_theme_selector_button_label: "Nsochie Họrọ Atụmatụ",
        close_filters_button_label: "Nsochie Nzacha",
        board_link: "Akụkọ Ndị",
        tags_link: "Mkpado",
        users_link: "Ndị Ọrụ",
        archive_link: "Akụkọ Ndị",
    },
};
