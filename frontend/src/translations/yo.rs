use super::{Text, Translation};

pub const YO: Translation<&'static str> = Translation {
    id: "yo",
    name: "YO - Yoruba",
    text: Text {
        to_do_column_title: "Ṣe Lati",
        in_progress_column_title: "Ninu Ilọsiwaju",
        done_column_title: "Ti ṣe",
        pick_language_tooltip: "Yan Ede",
        toggle_show_themes_tooltip: "Yipada Àkọsílẹ",
        toggle_dense_view_tooltip: "Yipada Akọsilẹ Idanimọ",
        edit_board_title_tooltip: "Ṣatunkọ Orukọ",
        board_title_input_label: "Orukọ",
        board_title_update_form_label: "Imudojuiwọn Akọsilẹ Orukọ",
        set_board_title_button_label: "Ṣeto Orukọ",
        cancel_board_title_update_button_label: "Fagile Imudojuiwọn Orukọ",
        task_title_input_label: "Orukọ",
        edit_task_title_tooltip: "Ṣatunkọ Orukọ iṣẹ",
        task_title_update_form_label: "Imudojuiwọn Orukọ Iṣẹ",
        set_task_title_button_label: "Ṣeto Orukọ Iṣẹ",
        cancel_task_title_update_button_label: "Fagile Imudojuiwọn Orukọ Iṣẹ",
        set_task_status_section_label: "Ṣeto Ipo Iṣẹ",
        to_do_button_tooltip: "Ṣe Lati",
        in_progress_button_tooltip: "Ninu Ilọsiwaju",
        done_button_tooltip: "Ti ṣe",
        task_actions_section_label: "Awọn Ise Išẹ",
        duplicate_task_button_tooltip: "Ṣe Afọwọkọ Iṣẹ",
        archive_task_button_tooltip: "Ṣeto Iṣẹ si ibi ipamọ",
        unarchive_task_button_tooltip: "Da Iṣẹ Pada",
        assignees_section_label: "Awọn Alabeere",
        assign_user_toggle_button_tooltip: "Fi Ọna fun Akọsilẹ",
        toggle_user_filter_button_label: "yi àlẹmọ olumulo",
        assignee_selection_section_label: "yan akọsilẹ",
        add_user_button_label: "Fi Ọlootu kun",
        add_user_form_label: "Fi Ọlootu kun",
        user_name_input_label: "Orukọ",
        cancel_adding_new_user_button_label: "fagile fifi olootu kun",
        remove_user_from_task_button_label: "yọ olootu kuro ninu iṣẹ",
        tags_section_label: "Awọn Ami",
        tag_selection_section_label: "Yan Ami",
        add_tag_button_label: "Fi Ami kun",
        add_tag_form_label: "Fi Ami kun",
        tag_name_input_label: "Orukọ",
        add_tag_toggle_button_tooltip: "Fi Ami kun",
        cancel_adding_new_tag_button_label: "fagile fifi ami kun",
        toggle_tag_filter_button_label: "yi àlẹmọ ami",
        remove_tag_from_task_button_label: "yọ ami kuro ninu iṣẹ",
        toggle_expand_task_button_label: "yi iṣẹ",
        due_date_section_label: " ọjọ ti o yẹ",
        edit_due_date_tooltip: "Ṣatunkọ ọjọ ti o yẹ",
        due_date_form_label: "fi ọjọ ti o yẹ",
        due_date_input_label: "jọjọ",
        set_due_date_button_label: "fi ọjọ ti o yẹ",
        cancel_due_date_update_button_label: "fagile imudojuiwọn ọjọ ti o yẹ",
        color_picker_legend_label: "Awọ",
        description_update_form_label: "imudojuiwọn apejuwe",
        set_description_button_label: "fi apejuwe si",
        cancel_description_update_button_label: "fagile imudojuiwọn apejuwe",
        bullet_points_button_tooltip: "Awọn ojuami Bullet",
        task_list_button_tooltip: "Akopọ Iṣẹ",
        description_text_area_label: "Apejuwe",
        description_section_label: "Apejuwe",
        edit_description_tooltip: "Ṣatunkọ Apejuwe",
        additional_actions_section_label: "iṣe afikun",
        delete_task_tooltip: "Paarẹ Iṣẹ",
        edit_tag_color_form_label: "Ṣatunkọ Awọ",
        edit_tag_color_button_label: "Ṣatunkọ Awọ",
        set_tag_color_button_label: "Fi Awọ kun",
        cancel_tag_color_update_label: "Fagile Imudojuiwọn Awọ",
        edit_tag_name_button_label: "Ṣatunkọ Orukọ",
        edit_tag_name_form_label: "Ṣatunkọ Orukọ",
        set_tag_name_button_label: "Fi Orukọ kun",
        cancel_tag_name_update_button_label: "Fagile Imudojuiwọn Orukọ",
        delete_tag_button_label: "Parẹ Ami",
        archive_tag_button_label: "Ṣatunkọ Ami si ibi ipamọ",
        unarchive_tag_button_label: "Da Ami Pada",
        edit_user_color_form_label: "Ṣatunkọ Awọ",
        set_user_color_button_label: "Fi Awọ Kun",
        cancel_user_color_update_button_label: "Fagile Imudojuiwọn Awọ",
        edit_user_color_button_label: "Ṣatunkọ Awọ",
        edit_user_name_form_label: "Ṣatunkọ Orukọ",
        set_user_name_button_label: "Fi Orukọ kun",
        cancel_user_name_update_button_label: "Fagile Imudojuiwọn Orukọ",
        edit_user_name_button_label: "Ṣatunkọ Orukọ",
        delete_user_button_label: "Parẹ Ọlootu",
        task_status_section_label: "Ipo Iṣẹ",
        filters_section_label: "Àlẹmọ",
        languages_section_title: "Yiyan Ede",
        custom_task_button_label: "Iṣẹ Adani",
        board_list_section_label: "Akopọ",
        join_board_button_label: "Darapọ mọ Akọsilẹ",
        create_new_board_button_label: "Ṣẹda Akọsilẹ Tuntun",
        or_label: "tabi",
        chat_gpt_limit_exceeded_title: "Fyẹn Awoyivẹ ChatGPT",
        chat_gpt_limit_exceeded_content: "O ti de opin awọn ipe ChatGPT. Jọwọ ṣe igbiyanju lẹẹkan si nigbamii.",
        chat_gpt_waiting_message: "Sọrọ pẹlu ChatGPT...",
        chat_gpt_error_title: "Aṣiṣe ChatGPT",
        chat_gpt_error_content: "Aṣiṣe kan ṣẹlẹ nigba gbiyanju lati sopọ si ChatGPT. Jọwọ ṣe igbiyanju lẹẹkan si nigbamii.",
        chat_gpt_prompt_input_title: "ChatGPT Ibi Pataki",
        chat_gpt_daily_attempts_left: "awọn igbiyanju ojoojumọ ti o ku",
        chat_gpt_prompt_input_content: "tabi yan ọkan ninu awọn imọran ni isalẹ:",
        chat_gpt_prompt_input_form_label: "chat gpt ibi pataki",
        chat_gpt_prompt_input_label: "Ibi Pataki:",
        suggest_cupcake_recipe_prompt: "tùtùìkè ìtọ\u{301} ṣe",
        paint_bedroom_prompt: "kun yara ibusun",
        friends_over_for_bbq_prompt: "awọn ọrẹ wa fun BBQ",
        prepare_for_rome_vacation_prompt: "pínpín fun isinmi Roma",
        house_tidy_prompt: "ile mọ",
        fix_fence_prompt: "Ṣatunṣe òdì",
        join_board_form_label: "Darapọ mọ Akọsilẹ",
        join_board_input_label: "Orukọ Akọsilẹ",
        cancel_joining_board_button_label: "Fagile Darapọ mọ Akọsilẹ",
        add_task_button_label: "Fi Iṣẹ Kun",
        remove_board_button_label: "Yọ Akọsilẹ",
        new_task_form_label: "Iṣẹ Tuntun",
        cancel_adding_new_task_button_label: "Fagile Fifi Iṣẹ Tuntun",
        navigation_section_label: "Lilọ kiri",
        toggle_actions_drawer_button_label: "Yi Iṣe Atẹgun",
        toggle_show_filters_button_label: "Yi àlẹmọ han",
        themes_section_label: "Ìṣè",
        close_theme_selector_button_label: "Pa Àmọ\u{301}wọ\u{301}le Ojú kan",
        close_filters_button_label: "Pa Àlẹmọ",
        board_link: "Akọsilẹ",
        tags_link: "Awọn Ami",
        users_link: "Awọn Ọlootu",
        archive_link: "Ibi ipamọ",
    },
};
