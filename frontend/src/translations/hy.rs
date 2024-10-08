use super::{Text, Translation};

pub const HY: Translation<&'static str> = Translation {
    id: "hy",
    name: "HY - Armenian",
    text: Text {
        to_do_column_title: "Ստեղծել",
        in_progress_column_title: "Ընթացքի մեջ",
        done_column_title: "Կատարված",
        pick_language_tooltip: "Ընտրել լեզուն",
        toggle_show_themes_tooltip: "Փոխել թեման",
        toggle_dense_view_tooltip: "Մանրացված տեսք",
        edit_board_title_tooltip: "Խմբագրել վերնագիրը",
        board_title_input_label: "Վերնագիր",
        board_title_update_form_label: "Թարմացնել վահանակի վերնագիրը",
        set_board_title_button_label: "Սահմանել վերնագիրը",
        cancel_board_title_update_button_label: "Չեղարկել վերնագիրը",
        task_title_input_label: "Վերնագիր",
        edit_task_title_tooltip: "Խմբագրել վերնագիրը",
        task_title_update_form_label: "Թարմացնել խնդիրների վերնագիրը",
        set_task_title_button_label: "Սահմանել վերնագիրը",
        cancel_task_title_update_button_label: "Չեղարկել վերնագիրը",
        set_task_status_section_label: "Սահմանել խնդիրների կարգավիճակը",
        to_do_button_tooltip: "Ստեղծել",
        in_progress_button_tooltip: "Ընթացքի մեջ",
        done_button_tooltip: "Կատարված",
        task_actions_section_label: "Խնդիրների գործողությունները",
        duplicate_task_button_tooltip: "Կրկնել խնդիրը",
        archive_task_button_tooltip: "Արխիվացնել խնդիրը",
        unarchive_task_button_tooltip: "Վերականգնել խնդիրը",
        assignees_section_label: "Պատասխանատուներ",
        assign_user_toggle_button_tooltip: "Մենք",
        toggle_user_filter_button_label: "Մմնտային ֆիլտր",
        assignee_selection_section_label: "Ընտրել պատասխանատուն",
        add_user_button_label: "Ավելացնել պատասխանատուն",
        add_user_form_label: "Ավելացնել պատասխանատուն",
        user_name_input_label: "Անուն",
        cancel_adding_new_user_button_label: "Չեղարկել",
        remove_user_from_task_button_label: "Հեռացնել պատասխանատուն",
        tags_section_label: "Պիտակներ",
        tag_selection_section_label: "Ընտրել պիտակը",
        add_tag_button_label: "Ավելացնել պիտակը",
        add_tag_form_label: "Ավելացնել պիտակը",
        tag_name_input_label: "Անուն",
        add_tag_toggle_button_tooltip: "Ավելացնել պիտակը",
        cancel_adding_new_tag_button_label: "Չեղարկել",
        toggle_tag_filter_button_label: "Ֆիլտր պիտակներով",
        remove_tag_from_task_button_label: "Հեռացնել պիտակը",
        toggle_expand_task_button_label: "Ընդլայնել խնդիրը",
        due_date_section_label: "Պահանջվող ժամկետը",
        edit_due_date_tooltip: "Խմբագրել ժամկետը",
        due_date_form_label: "Սահմանել ժամկետը",
        due_date_input_label: "Ժամկետ",
        set_due_date_button_label: "Սահմանել ժամկետը",
        cancel_due_date_update_button_label: "Չեղարկել ժամկետը",
        color_picker_legend_label: "Գույն",
        description_update_form_label: "Թարմացնել նկարագրությունը",
        set_description_button_label: "Սահմանել նկարագրությունը",
        cancel_description_update_button_label: "Չեղարկել նկարագրությանը",
        bullet_points_button_tooltip: "Կետագրումներ",
        task_list_button_tooltip: "Խնդիրների ցուցակ",
        description_text_area_label: "Նկարագրություն",
        description_section_label: "Նկարագրություն",
        edit_description_tooltip: "Խմբագրել նկարագրությունը",
        additional_actions_section_label: "Լրացուցիչ գործողություններ",
        delete_task_tooltip: "Ջնջել խնդիրը",
        edit_tag_color_form_label: "Խմբագրել գույնը",
        edit_tag_color_button_label: "Խմբագրել գույնը",
        set_tag_color_button_label: "Սահմանել գույնը",
        cancel_tag_color_update_label: "Չեղարկել գույնը",
        edit_tag_name_button_label: "Խմբագրել անունը",
        edit_tag_name_form_label: "Խմբագրել անունը",
        set_tag_name_button_label: "Սահմանել անունը",
        cancel_tag_name_update_button_label: "Չեղարկել անունը",
        delete_tag_button_label: "Ջնջել պիտակը",
        archive_tag_button_label: "Արխիվացնել պիտակը",
        unarchive_tag_button_label: "Վերականգնել պիտակը",
        edit_user_color_form_label: "Խմբագրել գույնը",
        set_user_color_button_label: "Սահմանել գույնը",
        cancel_user_color_update_button_label: "Չեղարկել գույնը",
        edit_user_color_button_label: "Խմբագրել գույնը",
        edit_user_name_form_label: "Խմբագրել անունը",
        set_user_name_button_label: "Սահմանել անունը",
        cancel_user_name_update_button_label: "Չեղարկել անունը",
        edit_user_name_button_label: "Խմբագրել անունը",
        delete_user_button_label: "Ջնջել պատասխանատուն",
        task_status_section_label: "Խնդիրների կարգավիճակը",
        filters_section_label: "Ֆիլտրեր",
        languages_section_title: "Լեզուներ",
        custom_task_button_label: "Պատվիրողական խնդիր",
        board_list_section_label: "Վահանակի ցուցակ",
        join_board_button_label: "Միանալ վահանակին",
        create_new_board_button_label: "Ստեղծել նոր վահանակ",
        or_label: "կամ",
        chat_gpt_limit_exceeded_title: "ChatGPT-ի սահմանը գերազանցված է",
        chat_gpt_limit_exceeded_content: "Դուք հասել եք ChatGPT-ի զանգերի սահմանը: Խնդրում ենք վերադառնալ ուշ:",
        chat_gpt_waiting_message: "Խոսում է ChatGPT...",
        chat_gpt_error_title: "ChatGPT սխալ",
        chat_gpt_error_content: "Միջտեսիլ կապվածության պրոբլեմ է տեղի ունեցել ChatGPT-ի հետ: Խնդրում ենք կրկին վերադարձել:",
        chat_gpt_prompt_input_title: "ChatGPT մտքի ցուցակ",
        chat_gpt_daily_attempts_left: "Օրվա փորձեր մնալ",
        chat_gpt_prompt_input_content: "կամ ընտրել մեկ այս ցուցակներից ներքևում:",
        chat_gpt_prompt_input_form_label: "chat gpt մտքի ցուցակ",
        chat_gpt_prompt_input_label: "Մտք:",
        suggest_cupcake_recipe_prompt: "առաջարկել կապկեյքի բաղադրատոմսը",
        paint_bedroom_prompt: "ներկել ննջասենյակը",
        friends_over_for_bbq_prompt: "ընկերների համբավառ BBQ",
        prepare_for_rome_vacation_prompt: "պատրաստել վարպետություն ասելու համար Հռոմի արձակուրդի առաջ",
        house_tidy_prompt: "տունը ծաղիկել",
        fix_fence_prompt: "պատրաստել ցանկապատը",
        join_board_form_label: "Միանալ վահանակին",
        join_board_input_label: "Վահանակի անունը",
        cancel_joining_board_button_label: "Չեղարկել միացումը վահանակին",
        add_task_button_label: "Ավելացնել խնդիրը",
        remove_board_button_label: "Ջնջել վահանակը",
        new_task_form_label: "Նոր խնդիր",
        cancel_adding_new_task_button_label: "Չեղարկել նոր խնդիրը",
        navigation_section_label: "Նավիգացիա",
        toggle_actions_drawer_button_label: "Գործողությունների ցուցակ",
        toggle_show_filters_button_label: "Ֆիլտրեր",
        themes_section_label: "Թեմաներ",
        close_theme_selector_button_label: "Փակել թեմայի ընտրիչը",
        close_filters_button_label: "Փակել ֆիլտրերը",
        board_link: "Վահանակ",
        tags_link: "Պիտակներ",
        users_link: "Օգտագործողներ",
        archive_link: "Արխիվ",
    },
};
