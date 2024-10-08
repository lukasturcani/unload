use super::{Text, Translation};

pub const TL: Translation<&'static str> = Translation {
    id: "tl",
    name: "TL - Tagalog",
    text: Text {
        to_do_column_title: "Gagawin",
        in_progress_column_title: "Ginagawa",
        done_column_title: "Tapos Na",
        pick_language_tooltip: "Pumili ng Wika",
        toggle_show_themes_tooltip: "Baguhin ang Tema",
        toggle_dense_view_tooltip: "I-toggle ang Dense View",
        edit_board_title_tooltip: "I-edit ang Pamagat",
        board_title_input_label: "Pamagat",
        board_title_update_form_label: "I-update ang Pamagat ng Board",
        set_board_title_button_label: "Itakda ang Pamagat",
        cancel_board_title_update_button_label: "Kanselahin ang Pag-update ng Pamagat",
        task_title_input_label: "Pamagat",
        edit_task_title_tooltip: "I-edit ang Pamagat",
        task_title_update_form_label: "I-update ang Pamagat ng Gawain",
        set_task_title_button_label: "Itakda ang Pamagat",
        cancel_task_title_update_button_label: "Kanselahin ang Pag-update ng Pamagat",
        set_task_status_section_label: "Itakda ang Status ng Gawain",
        to_do_button_tooltip: "Gagawin",
        in_progress_button_tooltip: "Ginagawa",
        done_button_tooltip: "Tapos Na",
        task_actions_section_label: "Mga Aksyon ng Gawain",
        duplicate_task_button_tooltip: "Dumikit na Gawain",
        archive_task_button_tooltip: "I-archive ang Gawain",
        unarchive_task_button_tooltip: "Ibalik ang Gawain",
        assignees_section_label: "Mga Asignado",
        assign_user_toggle_button_tooltip: "I-assign ang User",
        toggle_user_filter_button_label: "i-toggle ang user filter",
        assignee_selection_section_label: "pagpili ng asignado",
        add_user_button_label: "Dagdagan ang User",
        add_user_form_label: "Dagdagan ang User",
        user_name_input_label: "Pangalan",
        cancel_adding_new_user_button_label: "kanselahin ang pagdagdag ng user",
        remove_user_from_task_button_label: "alisin ang user sa gawain",
        tags_section_label: "Mga Tag",
        tag_selection_section_label: "Pagpili ng Tag",
        add_tag_button_label: "Dagdagan ang Tag",
        add_tag_form_label: "Dagdagan ang Tag",
        tag_name_input_label: "Pangalan",
        add_tag_toggle_button_tooltip: "Dagdagan ang Tag",
        cancel_adding_new_tag_button_label: "kanselahin ang pagdagdag ng tag",
        toggle_tag_filter_button_label: "i-toggle ang tag filter",
        remove_tag_from_task_button_label: "alisin ang tag sa gawain",
        toggle_expand_task_button_label: "i-toggle ang pag-expand ng gawain",
        due_date_section_label: "takdang petsa",
        edit_due_date_tooltip: "I-edit ang Takdang Petsa",
        due_date_form_label: "itakda ang takdang petsa",
        due_date_input_label: "Takdang Petsa",
        set_due_date_button_label: "itakda ang takdang petsa",
        cancel_due_date_update_button_label: "kanselahin ang pag-update ng takdang petsa",
        color_picker_legend_label: "Kulay",
        description_update_form_label: "i-update ang paglalarawan",
        set_description_button_label: "itakda ang paglalarawan",
        cancel_description_update_button_label: "kanselahin ang pag-update ng paglalarawan",
        bullet_points_button_tooltip: "Mga Bullet Point",
        task_list_button_tooltip: "Listahan ng Gawain",
        description_text_area_label: "Paglalarawan",
        description_section_label: "Paglalarawan",
        edit_description_tooltip: "I-edit ang Paglalarawan",
        additional_actions_section_label: "karagdagang aksyon",
        delete_task_tooltip: "Tanggalin ang Gawain",
        edit_tag_color_form_label: "I-edit ang Kulay",
        edit_tag_color_button_label: "I-edit ang Kulay",
        set_tag_color_button_label: "Itakda ang Kulay",
        cancel_tag_color_update_label: "Kanselahin ang Pag-update ng Kulay",
        edit_tag_name_button_label: "I-edit ang Pangalan",
        edit_tag_name_form_label: "I-edit ang Pangalan",
        set_tag_name_button_label: "Itakda ang Pangalan",
        cancel_tag_name_update_button_label: "Kanselahin ang Pag-update ng Pangalan",
        delete_tag_button_label: "Tanggalin ang Tag",
        archive_tag_button_label: "I-archive ang Tag",
        unarchive_tag_button_label: "Ibalik ang Tag",
        edit_user_color_form_label: "I-edit ang Kulay",
        set_user_color_button_label: "Itakda ang Kulay",
        cancel_user_color_update_button_label: "Kanselahin ang Pag-update ng Kulay",
        edit_user_color_button_label: "I-edit ang Kulay",
        edit_user_name_form_label: "I-edit ang Pangalan",
        set_user_name_button_label: "Itakda ang Pangalan",
        cancel_user_name_update_button_label: "Kanselahin ang Pag-update ng Pangalan",
        edit_user_name_button_label: "I-edit ang Pangalan",
        delete_user_button_label: "Tanggalin ang User",
        task_status_section_label: "Status ng Gawain",
        filters_section_label: "Mga Filter",
        languages_section_title: "Mga Wika",
        custom_task_button_label: "Custom na Gawain",
        board_list_section_label: "Listahan ng Board",
        join_board_button_label: "Sumali sa Board",
        create_new_board_button_label: "Lumikha ng Bagong Board",
        or_label: "o",
        chat_gpt_limit_exceeded_title: "Naabot ang Limitasyon ng ChatGPT",
        chat_gpt_limit_exceeded_content: "Naabot mo na ang limitasyon ng mga tawag sa ChatGPT. Mangyaring subukan muli mamaya.",
        chat_gpt_waiting_message: "Nakikipag-usap sa ChatGPT...",
        chat_gpt_error_title: "Error sa ChatGPT",
        chat_gpt_error_content: "Nagkaroon ng error habang sinusubukang kumonekta sa ChatGPT. Mangyaring subukan muli mamaya.",
        chat_gpt_prompt_input_title: "Prompt ng ChatGPT",
        chat_gpt_daily_attempts_left: "natitirang pang-araw-araw na pagtatangka",
        chat_gpt_prompt_input_content: "o pumili ng isa mula sa mga mungkahi sa ibaba:",
        chat_gpt_prompt_input_form_label: "chat gpt prompt",
        chat_gpt_prompt_input_label: "Prompt:",
        suggest_cupcake_recipe_prompt: "mungkahi ng recipe para sa cupcake",
        paint_bedroom_prompt: "pinturahan ang silid-tulugan",
        friends_over_for_bbq_prompt: "mga kaibigan na darating para sa BBQ",
        prepare_for_rome_vacation_prompt: "maghanda para sa bakasyon sa Roma",
        house_tidy_prompt: "ayusin ang bahay",
        fix_fence_prompt: "ayusin ang bakod",
        join_board_form_label: "Sumali sa Board",
        join_board_input_label: "Pangalan ng Board",
        cancel_joining_board_button_label: "Kanselahin ang Pagsali sa Board",
        add_task_button_label: "Dagdagan ang Gawain",
        remove_board_button_label: "Tanggalin ang Board",
        new_task_form_label: "Bagong Gawain",
        cancel_adding_new_task_button_label: "Kanselahin ang Bagong Gawain",
        navigation_section_label: "Navigasyon",
        toggle_actions_drawer_button_label: "I-toggle ang Actions Drawer",
        toggle_show_filters_button_label: "I-toggle ang Pagpakita ng mga Filter",
        themes_section_label: "Mga Tema",
        close_theme_selector_button_label: "Isara ang Theme Selector",
        close_filters_button_label: "Isara ang Mga Filter",
        board_link: "Board",
        tags_link: "Mga Tag",
        users_link: "Mga User",
        archive_link: "Archive",
    },
};
