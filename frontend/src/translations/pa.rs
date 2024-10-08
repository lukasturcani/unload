use super::{Text, Translation};

pub const PA: Translation<&'static str> = Translation {
    id: "pa",
    name: "PA - ਪ\u{a70}ਜਾਬੀ",
    text: Text {
        to_do_column_title: "ਕਰਨ\u{a47} ਵਾਲ\u{a47} ਕ\u{a70}ਮ",
        in_progress_column_title: "ਚਾਲ\u{a42} ਕ\u{a70}ਮ",
        done_column_title: "ਮ\u{a41}ਕ\u{a70}ਮਲ",
        pick_language_tooltip: "ਭਾਸ\u{a3c}ਾ ਚ\u{a41}ਣ\u{a4b}",
        toggle_show_themes_tooltip: "ਥੀਮ ਬਦਲ\u{a4b}",
        toggle_dense_view_tooltip: "ਘਣਾ ਦ\u{a4d}ਰਿਸ\u{a3c} ਬਦਲ\u{a4b}",
        edit_board_title_tooltip: "ਸਿਰਲ\u{a47}ਖ ਸ\u{a4b}ਧ\u{a4b}",
        board_title_input_label: "ਸਿਰਲ\u{a47}ਖ",
        board_title_update_form_label: "ਬ\u{a4b}ਰਡ ਸਿਰਲ\u{a47}ਖ ਨਵੀਨਿਆਉ",
        set_board_title_button_label: "ਸਿਰਲ\u{a47}ਖ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_board_title_update_button_label: "ਸਿਰਲ\u{a47}ਖ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        task_title_input_label: "ਸਿਰਲ\u{a47}ਖ",
        edit_task_title_tooltip: "ਸਿਰਲ\u{a47}ਖ ਸ\u{a4b}ਧ\u{a4b}",
        task_title_update_form_label: "ਕ\u{a70}ਮ ਸਿਰਲ\u{a47}ਖ ਨਵੀਨਿਆਉ",
        set_task_title_button_label: "ਸਿਰਲ\u{a47}ਖ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_task_title_update_button_label: "ਸਿਰਲ\u{a47}ਖ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        set_task_status_section_label: "ਕ\u{a70}ਮ ਦੀ ਸਥਿਤੀ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        to_do_button_tooltip: "ਕਰਨ\u{a47} ਵਾਲ\u{a47} ਕ\u{a70}ਮ",
        in_progress_button_tooltip: "ਚਾਲ\u{a42} ਕ\u{a70}ਮ",
        done_button_tooltip: "ਮ\u{a41}ਕ\u{a70}ਮਲ",
        task_actions_section_label: "ਕ\u{a70}ਮ ਦੀਆ\u{a02} ਕਾਰਵਾਈਆ\u{a02}",
        duplicate_task_button_tooltip: "ਕ\u{a70}ਮ ਦੀ ਨਕਲ ਕਰ\u{a4b}",
        archive_task_button_tooltip: "ਕ\u{a70}ਮ ਆਰਕਾਈਵ ਕਰ\u{a4b}",
        unarchive_task_button_tooltip: "ਕ\u{a70}ਮ ਬਹਾਲ ਕਰ\u{a4b}",
        assignees_section_label: "ਸਪ\u{a41}ਰਦਗੀਆ\u{a02}",
        assign_user_toggle_button_tooltip: "ਉਪਭ\u{a4b}ਗਤਾ ਸ\u{a4c}\u{a02}ਪ\u{a4b}",
        toggle_user_filter_button_label: "ਉਪਭ\u{a4b}ਗਤਾ ਫਿਲਟਰ ਬਦਲ\u{a4b}",
        assignee_selection_section_label: "ਸਪ\u{a41}ਰਦਗੀ ਚ\u{a4b}ਣ",
        add_user_button_label: "ਉਪਭ\u{a4b}ਗਤਾ ਜ\u{a4b}ੜ\u{a4b}",
        add_user_form_label: "ਉਪਭ\u{a4b}ਗਤਾ ਜ\u{a4b}ੜ\u{a4b}",
        user_name_input_label: "ਨਾਮ",
        cancel_adding_new_user_button_label: "ਨਵਾ\u{a02} ਉਪਭ\u{a4b}ਗਤਾ ਜ\u{a4b}ੜਨਾ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        remove_user_from_task_button_label: "ਕ\u{a70}ਮ ਤ\u{a4b}\u{a02} ਉਪਭ\u{a4b}ਗਤਾ ਹਟਾਓ",
        tags_section_label: "ਟ\u{a48}ਗ",
        tag_selection_section_label: "ਟ\u{a48}ਗ ਚ\u{a4b}ਣ",
        add_tag_button_label: "ਟ\u{a48}ਗ ਜ\u{a4b}ੜ\u{a4b}",
        add_tag_form_label: "ਟ\u{a48}ਗ ਜ\u{a4b}ੜ\u{a4b}",
        tag_name_input_label: "ਨਾਮ",
        add_tag_toggle_button_tooltip: "ਟ\u{a48}ਗ ਜ\u{a4b}ੜ\u{a4b}",
        cancel_adding_new_tag_button_label: "ਟ\u{a48}ਗ ਜ\u{a4b}ੜਨਾ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        toggle_tag_filter_button_label: "ਟ\u{a48}ਗ ਫਿਲਟਰ ਬਦਲ\u{a4b}",
        remove_tag_from_task_button_label: "ਕ\u{a70}ਮ ਤ\u{a4b}\u{a02} ਟ\u{a48}ਗ ਹਟਾਓ",
        toggle_expand_task_button_label: "ਕ\u{a70}ਮ ਵਧਾਓ ਬਦਲ\u{a4b}",
        due_date_section_label: "ਅ\u{a70}ਤਿਮ ਤਾਰੀਖ",
        edit_due_date_tooltip: "ਅ\u{a70}ਤਿਮ ਤਾਰੀਖ ਸ\u{a4b}ਧ\u{a4b}",
        due_date_form_label: "ਅ\u{a70}ਤਿਮ ਤਾਰੀਖ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        due_date_input_label: "ਅ\u{a70}ਤਿਮ ਤਾਰੀਖ",
        set_due_date_button_label: "ਅ\u{a70}ਤਿਮ ਤਾਰੀਖ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_due_date_update_button_label: "ਅ\u{a70}ਤਿਮ ਤਾਰੀਖ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        color_picker_legend_label: "ਰ\u{a70}ਗ",
        description_update_form_label: "ਵ\u{a47}ਰਵਾ ਨਵੀਨਿਆਉ",
        set_description_button_label: "ਵ\u{a47}ਰਵਾ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_description_update_button_label: "ਵ\u{a47}ਰਵਾ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        bullet_points_button_tooltip: "ਬ\u{a41}ਲ\u{a47}ਟ ਪ\u{a4b}ਇ\u{a70}ਟਸ",
        task_list_button_tooltip: "ਕ\u{a70}ਮ ਸ\u{a42}ਚੀ",
        description_text_area_label: "ਵ\u{a47}ਰਵਾ",
        description_section_label: "ਵ\u{a47}ਰਵਾ",
        edit_description_tooltip: "ਵ\u{a47}ਰਵਾ ਸ\u{a4b}ਧ\u{a4b}",
        additional_actions_section_label: "ਵਾਧ\u{a42} ਕਾਰਵਾਈਆ\u{a02}",
        delete_task_tooltip: "ਕ\u{a70}ਮ ਮਿਟਾਓ",
        edit_tag_color_form_label: "ਰ\u{a70}ਗ ਸ\u{a4b}ਧ\u{a4b}",
        edit_tag_color_button_label: "ਰ\u{a70}ਗ ਸ\u{a4b}ਧ\u{a4b}",
        set_tag_color_button_label: "ਰ\u{a70}ਗ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_tag_color_update_label: "ਰ\u{a70}ਗ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        edit_tag_name_button_label: "ਨਾਮ ਸ\u{a4b}ਧ\u{a4b}",
        edit_tag_name_form_label: "ਨਾਮ ਸ\u{a4b}ਧ\u{a4b}",
        set_tag_name_button_label: "ਨਾਮ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_tag_name_update_button_label: "ਨਾਮ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        delete_tag_button_label: "ਟ\u{a48}ਗ ਮਿਟਾਓ",
        archive_tag_button_label: "ਟ\u{a48}ਗ ਆਰਕਾਈਵ ਕਰ\u{a4b}",
        unarchive_tag_button_label: "ਟ\u{a48}ਗ ਬਹਾਲ ਕਰ\u{a4b}",
        edit_user_color_form_label: "ਰ\u{a70}ਗ ਸ\u{a4b}ਧ\u{a4b}",
        set_user_color_button_label: "ਰ\u{a70}ਗ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_user_color_update_button_label: "ਰ\u{a70}ਗ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        edit_user_color_button_label: "ਰ\u{a70}ਗ ਸ\u{a4b}ਧ\u{a4b}",
        edit_user_name_form_label: "ਨਾਮ ਸ\u{a4b}ਧ\u{a4b}",
        set_user_name_button_label: "ਨਾਮ ਸ\u{a48}ਟ ਕਰ\u{a4b}",
        cancel_user_name_update_button_label: "ਨਾਮ ਨਵੀਨਿਆਉ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        edit_user_name_button_label: "ਨਾਮ ਸ\u{a4b}ਧ\u{a4b}",
        delete_user_button_label: "ਉਪਭ\u{a4b}ਗਤਾ ਮਿਟਾਓ",
        task_status_section_label: "ਕ\u{a70}ਮ ਦੀ ਸਥਿਤੀ",
        filters_section_label: "ਫਿਲਟਰ",
        languages_section_title: "ਭਾਸ\u{a3c}ਾਵਾ\u{a02}",
        custom_task_button_label: "ਕਸਟਮ ਕ\u{a70}ਮ",
        board_list_section_label: "ਬ\u{a4b}ਰਡ ਸ\u{a42}ਚੀ",
        join_board_button_label: "ਬ\u{a4b}ਰਡ ਵਿ\u{a71}ਚ ਸ\u{a3c}ਾਮਿਲ ਹ\u{a4b}ਵ\u{a4b}",
        create_new_board_button_label: "ਨਵਾ\u{a02} ਬ\u{a4b}ਰਡ ਬਣਾਓ",
        or_label: "ਜਾ\u{a02}",
        chat_gpt_limit_exceeded_title: "ChatGPT ਸੀਮਾ ਪਾਰ ਗਈ",
        chat_gpt_limit_exceeded_content: "ਤ\u{a41}ਸੀ\u{a02} ChatGPT ਕਾਲਾ\u{a02} ਦੀ ਸੀਮਾ ਪਾਰ ਕਰ ਚ\u{a41}\u{a71}ਕ\u{a47} ਹ\u{a4b}। ਕਿਰਪਾ ਕਰਕ\u{a47} ਬਾਅਦ ਵਿ\u{a71}ਚ ਫਿਰ ਕ\u{a4b}ਸ\u{a3c}ਿਸ\u{a3c} ਕਰ\u{a4b}।",
        chat_gpt_waiting_message: "ChatGPT ਨਾਲ ਗ\u{a71}ਲਬਾਤ ਹ\u{a4b} ਰਹੀ ਹ\u{a48}...",
        chat_gpt_error_title: "ChatGPT ਗਲਤੀ",
        chat_gpt_error_content: "ChatGPT ਨਾਲ ਕਨ\u{a48}ਕਟ ਕਰਨ ਦੀ ਕ\u{a4b}ਸ\u{a3c}ਿਸ\u{a3c} ਦ\u{a4c}ਰਾਨ ਇ\u{a71}ਕ ਗਲਤੀ ਆਈ। ਕਿਰਪਾ ਕਰਕ\u{a47} ਬਾਅਦ ਵਿ\u{a71}ਚ ਫਿਰ ਕ\u{a4b}ਸ\u{a3c}ਿਸ\u{a3c} ਕਰ\u{a4b}।",
        chat_gpt_prompt_input_title: "ChatGPT ਪ\u{a4d}ਰਾ\u{a02}ਪਟ",
        chat_gpt_daily_attempts_left: "ਰ\u{a4b}ਜ\u{a3c}ਾਨਾ ਦੀਆ\u{a02} ਬਾਕੀ ਕ\u{a4b}ਸ\u{a3c}ਿਸ\u{a3c}ਾ\u{a02}",
        chat_gpt_prompt_input_content: "ਜਾ\u{a02} ਹ\u{a47}ਠਾ\u{a02} ਦਿ\u{a71}ਤੀਆ\u{a02} ਸ\u{a41}ਝਾਵਾ\u{a02} ਵਿ\u{a71}ਚ\u{a4b}\u{a02} ਇ\u{a71}ਕ ਚ\u{a41}ਣ ਕਰ\u{a4b}:",
        chat_gpt_prompt_input_form_label: "chat gpt ਪ\u{a4d}ਰਾ\u{a02}ਪਟ",
        chat_gpt_prompt_input_label: "ਪ\u{a4d}ਰਾ\u{a02}ਪਟ:",
        suggest_cupcake_recipe_prompt: "cupcake ਵਿਦੀ ਸ\u{a41}ਝਾਓ",
        paint_bedroom_prompt: "ਬ\u{a48}ਡਰ\u{a42}ਮ ਪ\u{a47}\u{a02}ਟ ਕਰ\u{a4b}",
        friends_over_for_bbq_prompt: "BBQ ਲਈ ਦ\u{a4b}ਸਤਾ\u{a02} ਨ\u{a42}\u{a70} ਸ\u{a71}ਦ\u{a4b}",
        prepare_for_rome_vacation_prompt: "ਰ\u{a4b}ਮ ਦੀ ਛ\u{a41}\u{a71}ਟੀਆ\u{a02} ਲਈ ਤਿਆਰੀ ਕਰ\u{a4b}",
        house_tidy_prompt: "ਘਰ ਸਾਫ\u{a3c} ਸ\u{a41}ਥਰਾ ਕਰ\u{a4b}",
        fix_fence_prompt: "ਬਾਢ ਬਨਾਓ",
        join_board_form_label: "ਬ\u{a4b}ਰਡ ਵਿਚ ਸ\u{a3c}ਾਮਿਲ ਹ\u{a4b}ਵ\u{a4b}",
        join_board_input_label: "ਬ\u{a4b}ਰਡ ਨਾਮ",
        cancel_joining_board_button_label: "ਬ\u{a4b}ਰਡ ਵਿ\u{a71}ਚ ਸ\u{a3c}ਾਮਿਲ ਹ\u{a4b}ਣਾ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        add_task_button_label: "ਕ\u{a70}ਮ ਜ\u{a4b}ੜ\u{a4b}",
        remove_board_button_label: "ਬ\u{a4b}ਰਡ ਹਟਾਓ",
        new_task_form_label: "ਨਵਾ\u{a02} ਕ\u{a70}ਮ",
        cancel_adding_new_task_button_label: "ਨਵਾ\u{a02} ਕ\u{a70}ਮ ਜ\u{a4b}ੜਨਾ ਰ\u{a71}ਦ ਕਰ\u{a4b}",
        navigation_section_label: "ਨ\u{a48}ਵੀਗ\u{a47}ਸ\u{a3c}ਨ",
        toggle_actions_drawer_button_label: "ਕਾਰਵਾਈਆ\u{a02} ਦਰਾਜ\u{a3c} ਬਦਲ\u{a4b}",
        toggle_show_filters_button_label: "ਫਿਲਟਰ ਬਦਲ\u{a4b}",
        themes_section_label: "ਥੀਮਾ\u{a02}",
        close_theme_selector_button_label: "ਥੀਮ ਸ\u{a48}ਲ\u{a47}ਕਟਰ ਬ\u{a70}ਦ ਕਰ\u{a4b}",
        close_filters_button_label: "ਫਿਲਟਰ ਬ\u{a70}ਦ ਕਰ\u{a4b}",
        board_link: "ਬ\u{a4b}ਰਡ",
        tags_link: "ਟ\u{a48}ਗ",
        users_link: "ਉਪਭ\u{a4b}ਗਤਾ",
        archive_link: "ਆਰਕਾਈਵ",
    },
};
