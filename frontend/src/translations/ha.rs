use super::{Text, Translation};

pub const HA: Translation<&'static str> = Translation {
    id: "ha",
    name: "HA - Hausa",
    text: Text {
        to_do_column_title: "Yi",
        in_progress_column_title: "Ana Gudanuwa",
        done_column_title: "Anyi",
        pick_language_tooltip: "Zaɓi yare",
        toggle_show_themes_tooltip: "Canza jigo",
        toggle_dense_view_tooltip: "Canza dandanawa mai yawa",
        edit_board_title_tooltip: "Gyara take",
        board_title_input_label: "Take",
        board_title_update_form_label: "Sabunta sunan allo",
        set_board_title_button_label: "Kafa take",
        cancel_board_title_update_button_label: "Soke sabunta taken",
        task_title_input_label: "Take",
        edit_task_title_tooltip: "Gyara take",
        task_title_update_form_label: "Sabunta taken aiki",
        set_task_title_button_label: "Kafa take",
        cancel_task_title_update_button_label: "Soke sabunta taken aiki",
        set_task_status_section_label: "Kafa matsayin aiki",
        to_do_button_tooltip: "Yi",
        in_progress_button_tooltip: "Ana Gudanuwa",
        done_button_tooltip: "Anyi",
        task_actions_section_label: "Matakan aiki",
        duplicate_task_button_tooltip: "Maƙala aiki",
        archive_task_button_tooltip: "Ajiye aiki",
        unarchive_task_button_tooltip: "Dawo da aiki",
        assignees_section_label: "Masu aikin",
        assign_user_toggle_button_tooltip: "Nadawa mai amfani",
        toggle_user_filter_button_label: "canza tace mai amfani",
        assignee_selection_section_label: "Zaɓin mai aiki",
        add_user_button_label: "Ƙara mai amfani",
        add_user_form_label: "Ƙara mai amfani",
        user_name_input_label: "Suna",
        cancel_adding_new_user_button_label: "soke ƙara mai amfani",
        remove_user_from_task_button_label: "cire mai amfani daga aiki",
        tags_section_label: "Alamomi",
        tag_selection_section_label: "Zaɓin alama",
        add_tag_button_label: "Ƙara alama",
        add_tag_form_label: "Ƙara alama",
        tag_name_input_label: "Suna",
        add_tag_toggle_button_tooltip: "Ƙara alama",
        cancel_adding_new_tag_button_label: "soke ƙara alama",
        toggle_tag_filter_button_label: "canza tace alama",
        remove_tag_from_task_button_label: "cire alama daga aiki",
        toggle_expand_task_button_label: "canza kumburi aikin",
        due_date_section_label: "ranar ƙarshe",
        edit_due_date_tooltip: "Gyara ranar ƙarshe",
        due_date_form_label: "kafa ranar ƙarshe",
        due_date_input_label: "Ran",
        set_due_date_button_label: "kafa ranar ƙarshe",
        cancel_due_date_update_button_label: "soke sabunta ranar ƙarshe",
        color_picker_legend_label: "Launi",
        description_update_form_label: "sabunta bayanin",
        set_description_button_label: "kafa bayanin",
        cancel_description_update_button_label: "soke sabunta bayanin",
        bullet_points_button_tooltip: "Ƙananan maki",
        task_list_button_tooltip: "Jerin ayyuka",
        description_text_area_label: "Bayanin",
        description_section_label: "Bayanin",
        edit_description_tooltip: "Gyara bayanin",
        additional_actions_section_label: "ƙarin ayyuka",
        delete_task_tooltip: "Share aiki",
        edit_tag_color_form_label: "Gyara launi",
        edit_tag_color_button_label: "Gyara launi",
        set_tag_color_button_label: "Kafa launi",
        cancel_tag_color_update_label: "Soke sabunta launi",
        edit_tag_name_button_label: "Gyara suna",
        edit_tag_name_form_label: "Gyara suna",
        set_tag_name_button_label: "Kafa suna",
        cancel_tag_name_update_button_label: "Soke sabunta suna",
        delete_tag_button_label: "Share alama",
        archive_tag_button_label: "Ajiye alama",
        unarchive_tag_button_label: "Dawo da alama",
        edit_user_color_form_label: "Gyara launi",
        set_user_color_button_label: "Kafa launi",
        cancel_user_color_update_button_label: "Soke sabunta launi",
        edit_user_color_button_label: "Gyara launi",
        edit_user_name_form_label: "Gyara suna",
        set_user_name_button_label: "Kafa suna",
        cancel_user_name_update_button_label: "Soke sabunta suna",
        edit_user_name_button_label: "Gyara suna",
        delete_user_button_label: "Share mai amfani",
        task_status_section_label: "Matsayin aiki",
        filters_section_label: "Tacewa",
        languages_section_title: "Yare",
        custom_task_button_label: "Keɓaɓɓen aiki",
        board_list_section_label: "Jerun allo",
        join_board_button_label: "Shiga allo",
        create_new_board_button_label: "Ƙirƙiri Sabon allo",
        or_label: "ko",
        chat_gpt_limit_exceeded_title: "ChatGPT ya wuce iyaka",
        chat_gpt_limit_exceeded_content:
            "Kun kai iyakar kiran ChatGPT. Da fatan za a gwada daga bisani.",
        chat_gpt_waiting_message: "Magana da ChatGPT...",
        chat_gpt_error_title: "ChatGPT kuskure",
        chat_gpt_error_content:
            "An samu kuskure yayin gwada haɗi zuwa Chat GPT. Da fatan za a gwada daga bisani.",
        chat_gpt_prompt_input_title: "ChatGPT tsokana",
        chat_gpt_daily_attempts_left: "kokarin yau da ya rage",
        chat_gpt_prompt_input_content: "ko zaɓi daya daga cikin shawarwarin da ke ƙasa:",
        chat_gpt_prompt_input_form_label: "chat gpt tsokana",
        chat_gpt_prompt_input_label: "Tsokana:",
        suggest_cupcake_recipe_prompt: "ba da shawarar girke-girke na ƙwaƙwalwa",
        paint_bedroom_prompt: "zane ɗakin kwana",
        friends_over_for_bbq_prompt: "abokai sun zo BBQ",
        prepare_for_rome_vacation_prompt: "shirya hutu na Roma",
        house_tidy_prompt: "tsabtace gida",
        fix_fence_prompt: "gyara shingen",
        join_board_form_label: "Shiga allo",
        join_board_input_label: "Sunan allo",
        cancel_joining_board_button_label: "Soke shiga allo",
        add_task_button_label: "Ƙara aiki",
        remove_board_button_label: "Cire allo",
        new_task_form_label: "Sabon aiki",
        cancel_adding_new_task_button_label: "Soke ƙara sabon aiki",
        navigation_section_label: "Nawigawo",
        toggle_actions_drawer_button_label: "Canza aljihunan ayyuka",
        toggle_show_filters_button_label: "Canza tacewa",
        themes_section_label: "Jigo",
        close_theme_selector_button_label: "Rufe zaɓin jigo",
        close_filters_button_label: "Rufe tacewa",
        board_link: "Allo",
        tags_link: "Alamomi",
        users_link: "Masu amfani",
        archive_link: "Ajiye",
    },
};
