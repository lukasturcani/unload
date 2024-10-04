use super::{Text, Translation};

pub const CS: Translation = Translation {
    id: "cs",
    name: "CS - Čeština",
    text: Text {
        to_do_column_title: "K dokončení",
        in_progress_column_title: "Probíhá",
        done_column_title: "Dokončeno",
        pick_language_tooltip: "Vybrat jazyk",
        toggle_show_themes_tooltip: "Změnit téma",
        toggle_dense_view_tooltip: "Přepnout kompaktní zobrazení",
        edit_board_title_tooltip: "Upravit název",
        board_title_input_label: "Název",
        board_title_update_form_label: "Aktualizovat název tabule",
        set_board_title_button_label: "Nastavit název",
        cancel_board_title_update_button_label: "Zrušit aktualizaci názvu",
        task_title_input_label: "Název",
        edit_task_title_tooltip: "Upravit název",
        task_title_update_form_label: "Aktualizovat název úkolu",
        set_task_title_button_label: "Nastavit název",
        cancel_task_title_update_button_label: "Zrušit aktualizaci názvu",
        set_task_status_section_label: "Nastavit stav úkolu",
        to_do_button_tooltip: "K dokončení",
        in_progress_button_tooltip: "Probíhá",
        done_button_tooltip: "Dokončeno",
        task_actions_section_label: "Akce úkolu",
        duplicate_task_button_tooltip: "Duplikovat úkol",
        archive_task_button_tooltip: "Archivovat úkol",
        unarchive_task_button_tooltip: "Obnovit úkol",
        assignees_section_label: "Přiřazení",
        assign_user_toggle_button_tooltip: "Přiřadit uživatele",
        toggle_user_filter_button_label: "Přepnout filtr uživatele",
        assignee_selection_section_label: "Výběr přiřazeného",
        add_user_button_label: "Přidat uživatele",
        add_user_form_label: "Přidat uživatele",
        cancel_adding_new_user_button_label: "Zrušit přidání uživatele",
        user_name_input_label: "Jméno",
        remove_user_from_task_button_label: "Odebrat uživatele z úkolu",
        tags_section_label: "Tagy",
        tag_selection_section_label: "Výběr tagů",
        add_tag_button_label: "Přidat tag",
        add_tag_form_label: "Přidat tag",
        tag_name_input_label: "Název",
        add_tag_toggle_button_tooltip: "Přidat tag",
        cancel_adding_new_tag_button_label: "Zrušit přidání tagu",
        toggle_tag_filter_button_label: "Přepnout filtr tagů",
        remove_tag_from_task_button_label: "Odebrat tag z úkolu",
        toggle_expand_task_button_label: "Rozbalit úkol",
        due_date_section_label: "Datum splnění",
        edit_due_date_tooltip: "Upravit datum splnění",
        due_date_form_label: "Nastavit datum splnění",
        due_date_input_label: "Termín",
        set_due_date_button_label: "Nastavit datum splnění",
        cancel_due_date_update_button_label: "Zrušit aktualizaci termínu",
        color_picker_legend_label: "Barva",
        description_update_form_label: "Aktualizovat popis",
        set_description_button_label: "Nastavit popis",
        cancel_description_update_button_label: "Zrušit aktualizaci popisu",
        bullet_points_button_tooltip: "Odrážky",
        task_list_button_tooltip: "Seznam úkolů",
        description_text_area_label: "Popis",
        description_section_label: "Popis",
        edit_description_tooltip: "Upravit popis",
        additional_actions_section_label: "Další akce",
        delete_task_tooltip: "Odstranit úkol",
        edit_tag_color_form_label: "Upravit barvu",
        edit_tag_color_button_label: "Upravit barvu",
        set_tag_color_button_label: "Nastavit barvu",
        cancel_tag_color_update_label: "Zrušit aktualizaci barvy",
        edit_tag_name_button_label: "Upravit název",
        edit_tag_name_form_label: "Upravit název",
        set_tag_name_button_label: "Nastavit název",
        cancel_tag_name_update_button_label: "Zrušit aktualizaci názvu",
        delete_tag_button_label: "Odstranit tag",
        archive_tag_button_label: "Archivovat tag",
        unarchive_tag_button_label: "Obnovit tag",
        edit_user_color_form_label: "Upravit barvu uživatele",
        set_user_color_button_label: "Nastavit barvu",
        cancel_user_color_update_button_label: "Zrušit aktualizaci barvy",
        edit_user_color_button_label: "Upravit barvu",
        edit_user_name_form_label: "Upravit jméno",
        set_user_name_button_label: "Nastavit jméno",
        cancel_user_name_update_button_label: "Zrušit aktualizaci jména",
        edit_user_name_button_label: "Upravit jméno",
        delete_user_button_label: "Odstranit uživatele",
        task_status_section_label: "Stav úkolu",
        filters_section_label: "Filtry",
        languages_section_title: "Jazyky",
        board_list_section_label: "Seznam tabulí",
        custom_task_button_label: "Vlastní úkol",
        join_board_button_label: "Připojit se k tabuli",
        create_new_board_button_label: "Vytvořit novou tabuli",
        chat_gpt_limit_exceeded_title: "Překročen limit ChatGPT",
        or_label: "nebo",
        chat_gpt_limit_exceeded_content:
            "Dosáhli jste limitu volání ChatGPT. Zkuste to prosím později.",
        chat_gpt_waiting_message: "Komunikace s ChatGPT...",
        chat_gpt_error_title: "Chyba ChatGPT",
        chat_gpt_error_content:
            "Při pokusu o připojení k ChatGPT došlo k chybě. Zkuste to prosím později.",
        chat_gpt_prompt_input_title: "Prompt ChatGPT",
        chat_gpt_daily_attempts_left: "zbývající denní pokusy",
        chat_gpt_prompt_input_content: "nebo vyberte jeden z níže uvedených návrhů:",
        chat_gpt_prompt_input_form_label: "Chat gpt prompt",
        suggest_cupcake_recipe_prompt: "navrhnout recept na cupcake",
        paint_bedroom_prompt: "vymalovat ložnici",
        friends_over_for_bbq_prompt: "přátelé na BBQ",
        prepare_for_rome_vacation_prompt: "připravit se na dovolenou v Římě",
        house_tidy_prompt: "uklidit dům",
        fix_fence_prompt: "opravit plot",
        chat_gpt_prompt_input_label: "Prompt:",
        join_board_form_label: "Připojit se k tabuli",
        join_board_input_label: "Název tabule",
        cancel_joining_board_button_label: "Zrušit připojení k tabuli",
        add_task_button_label: "Přidat úkol",
        remove_board_button_label: "Odstranit tabuli",
        new_task_form_label: "Nový úkol",
        cancel_adding_new_task_button_label: "Zrušit přidání úkolu",
        navigation_section_label: "Navigace",
        toggle_actions_drawer_button_label: "Přepnout panel akcí",
        toggle_show_filters_button_label: "Přepnout zobrazení filtrů",
        themes_section_label: "Témata",
        close_theme_selector_button_label: "Zavřít výběr tématu",
        close_filters_button_label: "Zavřít filtry",
        board_link: "Tabule",
        tags_link: "Tagy",
        users_link: "Uživatelé",
        archive_link: "Archiv",
    },
};