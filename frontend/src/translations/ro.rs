use super::{Text, Translation};

pub const RO: Translation<&'static str> = Translation {
    id: "ro",
    name: "RO - Română",
    text: Text {
        to_do_column_title: "De Făcut",
        in_progress_column_title: "În Progres",
        done_column_title: "Gata",
        pick_language_tooltip: "Alege Limba",
        toggle_show_themes_tooltip: "Schimbă Tema",
        toggle_dense_view_tooltip: "Comută Vizualizare Densă",
        edit_board_title_tooltip: "Editează Titlul",
        board_title_input_label: "Titlu",
        board_title_update_form_label: "Actualizează Titlul Panoului",
        set_board_title_button_label: "Setează Titlul",
        cancel_board_title_update_button_label: "Anulează Actualizarea Titlului",
        task_title_input_label: "Titlu",
        edit_task_title_tooltip: "Editează Titlul",
        task_title_update_form_label: "Actualizează Titlul Sarcinii",
        set_task_title_button_label: "Setează Titlul",
        cancel_task_title_update_button_label: "Anulează Actualizarea Titlului",
        set_task_status_section_label: "Setează Starea Sarcinii",
        to_do_button_tooltip: "De Făcut",
        in_progress_button_tooltip: "În Progres",
        done_button_tooltip: "Gata",
        task_actions_section_label: "Acțiuni Sarcini",
        duplicate_task_button_tooltip: "Dublează Sarcina",
        archive_task_button_tooltip: "Arhivează Sarcina",
        unarchive_task_button_tooltip: "Restaurează Sarcina",
        assignees_section_label: "Responsabili",
        assign_user_toggle_button_tooltip: "Atribuie Utilizator",
        toggle_user_filter_button_label: "comută filtru utilizator",
        assignee_selection_section_label: "selecție responsabil",
        add_user_button_label: "Adaugă Utilizator",
        add_user_form_label: "Adaugă Utilizator",
        user_name_input_label: "Nume",
        cancel_adding_new_user_button_label: "anulează adăugarea utilizatorului",
        remove_user_from_task_button_label: "elimină utilizatorul din sarcină",
        tags_section_label: "Etichete",
        tag_selection_section_label: "Selecție Etichetă",
        add_tag_button_label: "Adaugă Etichetă",
        add_tag_form_label: "Adaugă Etichetă",
        tag_name_input_label: "Nume",
        add_tag_toggle_button_tooltip: "Adaugă Etichetă",
        cancel_adding_new_tag_button_label: "anulează adăugarea etichetei",
        toggle_tag_filter_button_label: "comută filtru etichetă",
        remove_tag_from_task_button_label: "elimină eticheta din sarcină",
        toggle_expand_task_button_label: "comută extinderea sarcinii",
        due_date_section_label: "data scadentă",
        edit_due_date_tooltip: "Editează Data Scadentă",
        due_date_form_label: "setează data scadentă",
        due_date_input_label: "Scadent",
        set_due_date_button_label: "setează data scadentă",
        cancel_due_date_update_button_label: "anulează actualizarea datei scadente",
        color_picker_legend_label: "Culoare",
        description_update_form_label: "actualizează descrierea",
        set_description_button_label: "setează descrierea",
        cancel_description_update_button_label: "anulează actualizarea descrierii",
        bullet_points_button_tooltip: "Puncte de Bază",
        task_list_button_tooltip: "Listă Sarcini",
        description_text_area_label: "Descriere",
        description_section_label: "Descriere",
        edit_description_tooltip: "Editează Descrierea",
        additional_actions_section_label: "acțiuni suplimentare",
        delete_task_tooltip: "Șterge Sarcina",
        edit_tag_color_form_label: "Editează Culoarea",
        edit_tag_color_button_label: "Editează Culoarea",
        set_tag_color_button_label: "Setează Culoarea",
        cancel_tag_color_update_label: "Anulează Actualizarea Culorii",
        edit_tag_name_button_label: "Editează Numele",
        edit_tag_name_form_label: "Editează Numele",
        set_tag_name_button_label: "Setează Numele",
        cancel_tag_name_update_button_label: "Anulează Actualizarea Numelui",
        delete_tag_button_label: "Șterge Eticheta",
        archive_tag_button_label: "Arhivează Eticheta",
        unarchive_tag_button_label: "Restaurează Eticheta",
        edit_user_color_form_label: "Editează Culoarea",
        set_user_color_button_label: "Setează Culoarea",
        cancel_user_color_update_button_label: "Anulează Actualizarea Culorii",
        edit_user_color_button_label: "Editează Culoarea",
        edit_user_name_form_label: "Editează Numele",
        set_user_name_button_label: "Setează Numele",
        cancel_user_name_update_button_label: "Anulează Actualizarea Numelui",
        edit_user_name_button_label: "Editează Numele",
        delete_user_button_label: "Șterge Utilizator",
        task_status_section_label: "Starea Sarcinii",
        filters_section_label: "Filtre",
        languages_section_title: "Limbi",
        custom_task_button_label: "Sarcină Personalizată",
        board_list_section_label: "Listă Panouri",
        join_board_button_label: "Alătură-te Panoului",
        create_new_board_button_label: "Creează Un Nou Panou",
        or_label: "sau",
        chat_gpt_limit_exceeded_title: "Limita ChatGPT Depășită",
        chat_gpt_limit_exceeded_content: "Ai atins limita de apeluri ChatGPT. Te rugăm să încerci mai târziu.",
        chat_gpt_waiting_message: "Discut cu ChatGPT...",
        chat_gpt_error_title: "Eroare ChatGPT",
        chat_gpt_error_content: "A apărut o eroare în timpul încercării de a te conecta la ChatGPT. Te rugăm să încerci mai târziu.",
        chat_gpt_prompt_input_title: "Prompt ChatGPT",
        chat_gpt_daily_attempts_left: "încercări zilnice rămase",
        chat_gpt_prompt_input_content: "sau alege una dintre sugestiile de mai jos:",
        chat_gpt_prompt_input_form_label: "prompt chat gpt",
        chat_gpt_prompt_input_label: "Prompt:",
        suggest_cupcake_recipe_prompt: "sugerează o rețetă de brioșe",
        paint_bedroom_prompt: "vopsește dormitorul",
        friends_over_for_bbq_prompt: "prieteni pentru BBQ",
        prepare_for_rome_vacation_prompt: "pregătește-te pentru vacanța la Roma",
        house_tidy_prompt: "curăță casa",
        fix_fence_prompt: "repară gardul",
        join_board_form_label: "Alătură-te Panoului",
        join_board_input_label: "Numele Panoului",
        cancel_joining_board_button_label: "Anulează Alăturarea la Panou",
        add_task_button_label: "Adaugă Sarcină",
        remove_board_button_label: "Șterge Panoul",
        new_task_form_label: "Sarcină Nouă",
        cancel_adding_new_task_button_label: "Anulează Adăugarea Sarcinii Noi",
        navigation_section_label: "Navigare",
        toggle_actions_drawer_button_label: "Comută Sertarul de Acțiuni",
        toggle_show_filters_button_label: "Comută Afișarea Filtrelor",
        themes_section_label: "Teme",
        close_theme_selector_button_label: "Închide Selectorul de Teme",
        close_filters_button_label: "Închide Filtrele",
        board_link: "Panou",
        tags_link: "Etichete",
        users_link: "Utilizatori",
        archive_link: "Arhivă",
    },
};
