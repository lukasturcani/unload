use super::{Text, Translation};

pub const HU: Translation<&'static str> = Translation {
    id: "hu",
    name: "HU - Hungarian",
    text: Text {
        to_do_column_title: "Teendők",
        in_progress_column_title: "Folyamatban",
        done_column_title: "Kész",
        pick_language_tooltip: "Nyelv kiválasztása",
        toggle_show_themes_tooltip: "Téma módosítása",
        toggle_dense_view_tooltip: "Tömör nézet",
        edit_board_title_tooltip: "Cím szerkesztése",
        board_title_input_label: "Cím",
        board_title_update_form_label: "Táblázat címének frissítése",
        set_board_title_button_label: "Cím beállítása",
        cancel_board_title_update_button_label: "Cím módosításának megszakítása",
        task_title_input_label: "Cím",
        edit_task_title_tooltip: "Cím szerkesztése",
        task_title_update_form_label: "Feladat címének frissítése",
        set_task_title_button_label: "Cím beállítása",
        cancel_task_title_update_button_label: "Módosítás megszakítása",
        set_task_status_section_label: "Feladat állapotának beállítása",
        to_do_button_tooltip: "Teendők",
        in_progress_button_tooltip: "Folyamatban",
        done_button_tooltip: "Kész",
        task_actions_section_label: "Feladat műveletei",
        duplicate_task_button_tooltip: "Feladat másolása",
        archive_task_button_tooltip: "Feladat archiválása",
        unarchive_task_button_tooltip: "Feladat visszaállítása",
        assignees_section_label: "Felelősök",
        assign_user_toggle_button_tooltip: "Felhasználó hozzárendelése",
        toggle_user_filter_button_label: "felhasználói szűrő megjelenítése",
        assignee_selection_section_label: "felhasználó kiválasztása",
        add_user_button_label: "Felhasználó hozzáadása",
        add_user_form_label: "Felhasználó hozzáadása",
        user_name_input_label: "Név",
        cancel_adding_new_user_button_label: "felhasználó hozzáadásának megszakítása",
        remove_user_from_task_button_label: "felhasználó eltávolítása a feladatról",
        tags_section_label: "Címkék",
        tag_selection_section_label: "Címke kiválasztása",
        add_tag_button_label: "Címke hozzáadása",
        add_tag_form_label: "Címke hozzáadása",
        tag_name_input_label: "Név",
        add_tag_toggle_button_tooltip: "Címke hozzáadása",
        cancel_adding_new_tag_button_label: "címke hozzáadásának megszakítása",
        toggle_tag_filter_button_label: "címkeszűrő megjelenítése",
        remove_tag_from_task_button_label: "címke eltávolítása a feladatról",
        toggle_expand_task_button_label: "feladat kibontása",
        due_date_section_label: "határidő",
        edit_due_date_tooltip: "Határidő szerkesztése",
        due_date_form_label: "határidő beállítása",
        due_date_input_label: "Határidő",
        set_due_date_button_label: "határidő beállítása",
        cancel_due_date_update_button_label: "határidő módosításának megszakítása",
        color_picker_legend_label: "Szín",
        description_update_form_label: "leírás frissítése",
        set_description_button_label: "leírás beállítása",
        cancel_description_update_button_label: "leírás frissítésének megszakítása",
        bullet_points_button_tooltip: "Felsorolás",
        task_list_button_tooltip: "Feladatlista",
        description_text_area_label: "Leírás",
        description_section_label: "Leírás",
        edit_description_tooltip: "Leírás szerkesztése",
        additional_actions_section_label: "további műveletek",
        delete_task_tooltip: "Feladat törlése",
        edit_tag_color_form_label: "Szín módosítása",
        edit_tag_color_button_label: "Szín módosítása",
        set_tag_color_button_label: "Szín beállítása",
        cancel_tag_color_update_label: "Szín módosításának megszakítása",
        edit_tag_name_button_label: "Név módosítása",
        edit_tag_name_form_label: "Név módosítása",
        set_tag_name_button_label: "Név beállítása",
        cancel_tag_name_update_button_label: "Név módosításának megszakítása",
        delete_tag_button_label: "Címke törlése",
        archive_tag_button_label: "Címke archiválása",
        unarchive_tag_button_label: "Címke visszaállítása",
        edit_user_color_form_label: "Szín módosítása",
        set_user_color_button_label: "Szín beállítása",
        cancel_user_color_update_button_label: "Szín módosításának megszakítása",
        edit_user_color_button_label: "Szín módosítása",
        edit_user_name_form_label: "Név módosítása",
        set_user_name_button_label: "Név beállítása",
        cancel_user_name_update_button_label: "Név módosításának megszakítása",
        edit_user_name_button_label: "Név módosítása",
        delete_user_button_label: "Felhasználó törlése",
        task_status_section_label: "Feladat állapota",
        filters_section_label: "Szűrők",
        languages_section_title: "Nyelvek",
        custom_task_button_label: "Egyéni feladat",
        board_list_section_label: "Táblák listája",
        join_board_button_label: "Csatlakozás a táblához",
        create_new_board_button_label: "Új tábla létrehozása",
        or_label: "vagy",
        chat_gpt_limit_exceeded_title: "ChatGPT kvóta túllépve",
        chat_gpt_limit_exceeded_content:
            "Elérted a ChatGPT hívások limitjét. Kérlek, próbáld újra később.",
        chat_gpt_waiting_message: "Beszélgetés a ChatGPT-vel...",
        chat_gpt_error_title: "ChatGPT hiba",
        chat_gpt_error_content:
            "Hiba történt a ChatGPT-hez való csatlakozás során. Kérlek, próbáld újra később.",
        chat_gpt_prompt_input_title: "ChatGPT prompt",
        chat_gpt_daily_attempts_left: "napi próbálkozások hátralévő száma",
        chat_gpt_prompt_input_content: "vagy válassz egyet az alábbi javaslatok közül:",
        chat_gpt_prompt_input_form_label: "chat gpt prompt",
        chat_gpt_prompt_input_label: "Prompt:",
        suggest_cupcake_recipe_prompt: "javasolj cupcake receptet",
        paint_bedroom_prompt: "hálószoba festése",
        friends_over_for_bbq_prompt: "barátok grillezéshez való meghívása",
        prepare_for_rome_vacation_prompt: "római nyaralás előkészítése",
        house_tidy_prompt: "ház rendbetétele",
        fix_fence_prompt: "kerítés javítása",
        join_board_form_label: "Csatlakozás a táblához",
        join_board_input_label: "Tábla neve",
        cancel_joining_board_button_label: "Csatlakozás megszakítása",
        add_task_button_label: "Feladat hozzáadása",
        remove_board_button_label: "Tábla eltávolítása",
        new_task_form_label: "Új feladat",
        cancel_adding_new_task_button_label: "Új feladat létrehozásának megszakítása",
        navigation_section_label: "Navigáció",
        toggle_actions_drawer_button_label: "Műveletek fiók megjelenítése",
        toggle_show_filters_button_label: "Szűrők megjelenítése",
        themes_section_label: "Témák",
        close_theme_selector_button_label: "Téma kiválasztásának bezárása",
        close_filters_button_label: "Szűrők bezárása",
        board_link: "Tábla",
        tags_link: "Címkék",
        users_link: "Felhasználók",
        archive_link: "Archívum",
    },
};
