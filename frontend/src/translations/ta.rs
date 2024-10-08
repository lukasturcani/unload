use super::{Text, Translation};

pub const TA: Translation<&'static str> = Translation {
    id: "ta",
    name: "TA - Tamil",
    text: Text {
        to_do_column_title: "செய\u{bcd}யவும\u{bcd}",
        in_progress_column_title: "மேல\u{bbe}ணையில\u{bcd}",
        done_column_title: "முடிந\u{bcd}தது",
        pick_language_tooltip: "மெ\u{bbe}ழியை தேர\u{bcd}ந\u{bcd}தெடுங\u{bcd}கள\u{bcd}",
        toggle_show_themes_tooltip: "த\u{bc0}ம\u{bcd}களை ம\u{bbe}ற\u{bcd}று",
        toggle_dense_view_tooltip: "கட\u{bcd}டுக\u{bcd} கோப\u{bcd}புகள\u{bcd} க\u{bbe}ட\u{bcd}சி முறை",
        edit_board_title_tooltip: "தலைப\u{bcd}பை திருத\u{bcd}த",
        board_title_input_label: "தலைப\u{bcd}பு",
        board_title_update_form_label: "பலகையில\u{bcd} தலைப\u{bcd}பை ம\u{bbe}ற\u{bcd}றவும\u{bcd}",
        set_board_title_button_label: "தலைப\u{bcd}பை அமைக\u{bcd}கவும\u{bcd}",
        cancel_board_title_update_button_label: "தலைப\u{bcd}பு புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        task_title_input_label: "தலைப\u{bcd}பு",
        edit_task_title_tooltip: "தலைப\u{bcd}பை திருத\u{bcd}த",
        task_title_update_form_label: "பணி தலைப\u{bcd}பை மேம\u{bcd}படுத\u{bcd}தவும\u{bcd}",
        set_task_title_button_label: "தலைப\u{bcd}பை அமைக\u{bcd}கவும\u{bcd}",
        cancel_task_title_update_button_label: "பணி தலைப\u{bcd}பு புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        set_task_status_section_label: "பணி நிலையை அமைக\u{bcd}கவும\u{bcd}",
        to_do_button_tooltip: "செய\u{bcd}யவும\u{bcd}",
        in_progress_button_tooltip: "மேல\u{bbe}ணையில\u{bcd}",
        done_button_tooltip: "முடிந\u{bcd}தது",
        task_actions_section_label: "பணி செயல\u{bcd}கள\u{bcd}",
        duplicate_task_button_tooltip: "பணியை நகலெடு",
        archive_task_button_tooltip: "பணியை க\u{bbe}ப\u{bcd}பிடு",
        unarchive_task_button_tooltip: "பனியைக\u{bcd} ம\u{bc0}ட\u{bcd}கவும\u{bcd}",
        assignees_section_label: "நியமனர\u{bcd}கள\u{bcd}",
        assign_user_toggle_button_tooltip: "பயனரை நியமிக\u{bcd}க",
        toggle_user_filter_button_label: "பயனர\u{bcd} வடிகட\u{bcd}டி இயக\u{bcd}கவும\u{bcd}",
        assignee_selection_section_label: "நியமிக\u{bcd}க முழுதும\u{bcd}",
        add_user_button_label: "பயனரை சேர\u{bcd}",
        add_user_form_label: "பயனரை சேர\u{bcd}",
        user_name_input_label: "பெயர\u{bcd}",
        cancel_adding_new_user_button_label: "புதிய பயனர\u{bcd} சேர\u{bcd}ப\u{bcd}பதை ரத\u{bcd}துசெய\u{bcd}",
        remove_user_from_task_button_label: "பணியிலிருந\u{bcd}து பயனரை ந\u{bc0}க\u{bcd}கவும\u{bcd}",
        tags_section_label: "உருபடிகள\u{bcd}",
        tag_selection_section_label: "உருபடிகளைத\u{bcd} தேர\u{bcd}வு",
        add_tag_button_label: "உருபடியை சேர\u{bcd}",
        add_tag_form_label: "உருபடியை சேர\u{bcd}",
        tag_name_input_label: "பெயர\u{bcd}",
        add_tag_toggle_button_tooltip: "உருபடியை சேர\u{bcd}",
        cancel_adding_new_tag_button_label: "உருப\u{bcd}படி சேர\u{bcd}ப\u{bcd}பதை ரத\u{bcd}துச\u{bcd} செய\u{bcd}",
        toggle_tag_filter_button_label: "உருபடி வடிகட\u{bcd}டி இயக\u{bcd}கவும",
        remove_tag_from_task_button_label: "பணியிலிருந\u{bcd}து உருப\u{bcd}படியை ந\u{bc0}க\u{bcd}கு",
        toggle_expand_task_button_label: "பணியை விரிவ\u{bbe}க\u{bcd}கு",
        due_date_section_label: "நியமிக\u{bcd}கப\u{bcd}பட\u{bcd}ட தேதி",
        edit_due_date_tooltip: "நியமிக\u{bcd}கப\u{bcd}பட\u{bcd}ட தேதியை திருத\u{bcd}து",
        due_date_form_label: "நியமிக\u{bcd}கப\u{bcd}பட\u{bcd}ட தேதியை அமைக\u{bcd}கவும\u{bcd}",
        due_date_input_label: "நியமிக\u{bcd}கப\u{bcd}பட\u{bcd}ட தேதி",
        set_due_date_button_label: "நியமிக\u{bcd}கப\u{bcd}பட\u{bcd}ட தேதியை அமைக\u{bcd}கவும\u{bcd}",
        cancel_due_date_update_button_label: "நியமிக\u{bcd}கப\u{bcd}பட\u{bcd}ட தேதி புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        color_picker_legend_label: "நிறம\u{bcd}",
        description_update_form_label: "விளக\u{bcd}கத\u{bcd}தை மேம\u{bcd}படுத\u{bcd}தவும\u{bcd}",
        set_description_button_label: "விளக\u{bcd}கம\u{bcd} அமைக\u{bcd}கவும\u{bcd}",
        cancel_description_update_button_label: "விளக\u{bcd}கம\u{bcd} புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        bullet_points_button_tooltip: "புள\u{bcd}ளி குறிகள\u{bcd}",
        task_list_button_tooltip: "பணி பட\u{bcd}டியல\u{bcd}",
        description_text_area_label: "விளக\u{bcd}கம\u{bcd}",
        description_section_label: "விளக\u{bcd}கம\u{bcd}",
        edit_description_tooltip: "விளக\u{bcd}கத\u{bcd}தை திருத\u{bcd}த",
        additional_actions_section_label: "கூடுதல\u{bcd} செயல\u{bcd}கள\u{bcd}",
        delete_task_tooltip: "பணியை ந\u{bc0}க\u{bcd}கு",
        edit_tag_color_form_label: "நிறத\u{bcd}தை திருத\u{bcd}து",
        edit_tag_color_button_label: "நிறத\u{bcd}தை திருத\u{bcd}து",
        set_tag_color_button_label: "நிறத\u{bcd}தை அமைக\u{bcd}கவும\u{bcd}",
        cancel_tag_color_update_label: "நிறத\u{bcd}தை புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        edit_tag_name_button_label: "பெயரை திருத\u{bcd}து",
        edit_tag_name_form_label: "பெயரை திருத\u{bcd}து",
        set_tag_name_button_label: "பெயரை அமைக\u{bcd}கவும\u{bcd}",
        cancel_tag_name_update_button_label: "பெயரை புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        delete_tag_button_label: "உருபடியை ந\u{bc0}க\u{bcd}கு",
        archive_tag_button_label: "உருப\u{bcd}பு க\u{bbe}ப\u{bcd}பிடம\u{bcd}",
        unarchive_tag_button_label: "உருப\u{bcd}படியை ம\u{bc0}ட\u{bcd}கவும\u{bcd}",
        edit_user_color_form_label: "நிறத\u{bcd}தை திருத\u{bcd}து",
        set_user_color_button_label: "நிறத\u{bcd}தை அமைக\u{bcd}கவும\u{bcd}",
        cancel_user_color_update_button_label: "நிறத\u{bcd}தை புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        edit_user_color_button_label: "நிறத\u{bcd}தை திருத\u{bcd}து",
        edit_user_name_form_label: "பெயரை திருத\u{bcd}து",
        set_user_name_button_label: "பெயரை அமைக\u{bcd}கவும\u{bcd}",
        cancel_user_name_update_button_label: "பெயரை புதுப\u{bcd}பிப\u{bcd}பை ரத\u{bcd}துசெய\u{bcd}",
        edit_user_name_button_label: "பெயரை திருத\u{bcd}து",
        delete_user_button_label: "பயனரை ந\u{bc0}க\u{bcd}கு",
        task_status_section_label: "பணி நிலை",
        filters_section_label: "வடிகளும\u{bcd}",
        languages_section_title: "மொழிகள\u{bcd}",
        custom_task_button_label: "தனிப\u{bcd}பயன\u{bcd} பணி",
        board_list_section_label: "பலகை பட\u{bcd}டியல\u{bcd}",
        join_board_button_label: "பலகையில\u{bcd} சேரவும\u{bcd}",
        create_new_board_button_label: "புதிய பலகையை உருவ\u{bbe}க\u{bcd}கு",
        or_label: "அல\u{bcd}லது",
        chat_gpt_limit_exceeded_title: "ChatGPT வரம\u{bcd}பு ம\u{bc0}றியுள\u{bcd}ளது",
        chat_gpt_limit_exceeded_content: "ChatGPT அழைப\u{bcd}புகளின\u{bcd} வரம\u{bcd}பை ந\u{bc0}ங\u{bcd}கள\u{bcd} அடைந\u{bcd}துவிட\u{bcd}ட\u{bc0}ர\u{bcd}கள\u{bcd}. क\u{943}पया பின\u{bcd}னர\u{bcd} மறு முயற\u{bcd}சி செய\u{bcd}யவும\u{bcd}.",
        chat_gpt_waiting_message: "ChatGPT உடன\u{bcd} பேசுதல\u{bcd}...",
        chat_gpt_error_title: "ChatGPT பிழை",
        chat_gpt_error_content: "ChatGPT-க\u{bcd}கு இணைப\u{bcd}பதற\u{bcd}கு முயற\u{bcd}சிக\u{bcd}கும\u{bcd}போது ஒரு பிழை ஏற\u{bcd}பட\u{bcd}டது. படுக\u{bcd}க\u{bbe}த\u{bcd} கட\u{bcd}டியும\u{bbe}ய\u{bcd}த\u{bcd} திரும\u{bcd}ப\u{bbe}து.",
        chat_gpt_prompt_input_title: "ChatGPT உத\u{bcd}தரவு",
        chat_gpt_daily_attempts_left: "ம\u{bbe}திரித\u{bcd} தூண\u{bcd}டி",
        chat_gpt_prompt_input_content: "அல\u{bcd}லது க\u{bc0}ழே உள\u{bcd}ள பரிந\u{bcd}துரைகளில\u{bcd} ஒன\u{bcd}றைத\u{bcd} தேர\u{bcd}வு செய\u{bcd}யவும\u{bcd}:",
        chat_gpt_prompt_input_form_label: "chat gpt ஊக\u{bcd}கம\u{bcd}",
        chat_gpt_prompt_input_label: "உத\u{bcd}தரவு:",
        suggest_cupcake_recipe_prompt: "கப\u{bcd}கேக\u{bcd} பற\u{bcd}ற\u{bbe} சமையல\u{bcd} தகவல\u{bcd} பரிந\u{bcd}துரை",
        paint_bedroom_prompt: "படுக\u{bcd}கை அறை",
        friends_over_for_bbq_prompt: "மற\u{bcd}றவைகளைப\u{bcd} பட\u{bcd}டியலிடுங\u{bcd}கள\u{bcd}",
        prepare_for_rome_vacation_prompt: "ரோமின\u{bcd} விடுமுறையை தய\u{bbe}ரிக\u{bcd}கவும\u{bcd}",
        house_tidy_prompt: "clean house",
        fix_fence_prompt: "விளக\u{bcd}கத\u{bcd}தை மேம\u{bcd}படுத\u{bcd}தவும\u{bcd}",
        join_board_form_label: "பலகையில\u{bcd} சேரவும\u{bcd}",
        join_board_input_label: "பலகை பெயர\u{bcd}",
        cancel_joining_board_button_label: "பலகையில\u{bcd} சேர\u{bcd}வதை ரத\u{bcd}து செய\u{bcd}தல\u{bcd}",
        add_task_button_label: "பணியைச\u{bcd} சேர\u{bcd}",
        remove_board_button_label: "பலகையை ந\u{bc0}க\u{bcd}கு",
        new_task_form_label: "புதிய பணி",
        cancel_adding_new_task_button_label: "புதிய பணியை சேர\u{bcd}ப\u{bcd}பதை ரத\u{bcd}து செய\u{bcd}தல\u{bcd}",
        navigation_section_label: "வழிபடுத\u{bcd}தல\u{bcd}",
        toggle_actions_drawer_button_label: "செயல\u{bcd}கள\u{bcd} ப\u{bbe}லத\u{bcd}தை திருக\u{bcd}கவும\u{bcd}",
        toggle_show_filters_button_label: "வடிகளின\u{bcd} க\u{bbe}ட\u{bcd}சியை இயக\u{bcd}கவும\u{bcd}",
        themes_section_label: "த\u{bc0}ம\u{bcd}கள\u{bcd}",
        close_theme_selector_button_label: "த\u{bc0}ம\u{bcd}களை மூடு",
        close_filters_button_label: "வடிகளின\u{bcd} க\u{bbe}ட\u{bcd}சியை மூடு",
        board_link: "பலகை",
        tags_link: "உருபடிகள\u{bcd}",
        users_link: "பயனர\u{bcd}கள\u{bcd}",
        archive_link: "க\u{bbe}ப\u{bcd}பிடம\u{bcd}",
    },
};
