use super::{Text, Translation};

pub const SV: Translation<&'static str> = Translation {
    id: "sv",
    name: "SV - Svenska",
    text: Text {
        to_do_column_title: "Att Göra",
        in_progress_column_title: "Pågår",
        done_column_title: "Klart",
        pick_language_tooltip: "Välj Språk",
        toggle_show_themes_tooltip: "Byt Tema",
        toggle_dense_view_tooltip: "Växla Tät Visning",
        edit_board_title_tooltip: "Redigera Titel",
        board_title_input_label: "Titel",
        board_title_update_form_label: "Uppdatera Tavlans Titel",
        set_board_title_button_label: "Sätt Titel",
        cancel_board_title_update_button_label: "Avbryt Titeluppdatering",
        task_title_input_label: "Titel",
        edit_task_title_tooltip: "Redigera Titel",
        task_title_update_form_label: "Uppdatera Uppgiftens Titel",
        set_task_title_button_label: "Sätt Titel",
        cancel_task_title_update_button_label: "Avbryt Titeluppdatering",
        set_task_status_section_label: "Sätt Uppgiftens Status",
        to_do_button_tooltip: "Att Göra",
        in_progress_button_tooltip: "Pågår",
        done_button_tooltip: "Klart",
        task_actions_section_label: "Uppgiftsåtgärder",
        duplicate_task_button_tooltip: "Duplicera Uppgift",
        archive_task_button_tooltip: "Arkivera Uppgift",
        unarchive_task_button_tooltip: "Återställ Uppgift",
        assignees_section_label: "Tilldelade",
        assign_user_toggle_button_tooltip: "Tilldela Användare",
        toggle_user_filter_button_label: "växla användarfilter",
        assignee_selection_section_label: "urval av tilldelade",
        add_user_button_label: "Lägg till Användare",
        add_user_form_label: "Lägg till Användare",
        user_name_input_label: "Namn",
        cancel_adding_new_user_button_label: "avbryt tillägg av användare",
        remove_user_from_task_button_label: "ta bort användare från uppgift",
        tags_section_label: "Taggar",
        tag_selection_section_label: "Urval av Taggar",
        add_tag_button_label: "Lägg Till Tagg",
        add_tag_form_label: "Lägg Till Tagg",
        tag_name_input_label: "Namn",
        add_tag_toggle_button_tooltip: "Lägg Till Tagg",
        cancel_adding_new_tag_button_label: "avbryt tillägg av tagg",
        toggle_tag_filter_button_label: "växla taggfilter",
        remove_tag_from_task_button_label: "ta bort tagg från uppgift",
        toggle_expand_task_button_label: "växla utvidga uppgift",
        due_date_section_label: "förfallodatum",
        edit_due_date_tooltip: "Redigera Förfallodatum",
        due_date_form_label: "sätt förfallodatum",
        due_date_input_label: "Förfaller",
        set_due_date_button_label: "sätt förfallodatum",
        cancel_due_date_update_button_label: "avbryt uppdatering av förfallodatum",
        color_picker_legend_label: "Färg",
        description_update_form_label: "uppdatera beskrivning",
        set_description_button_label: "sätt beskrivning",
        cancel_description_update_button_label: "avbryt uppdatering av beskrivning",
        bullet_points_button_tooltip: "Punkter",
        task_list_button_tooltip: "Uppgiftslista",
        description_text_area_label: "Beskrivning",
        description_section_label: "Beskrivning",
        edit_description_tooltip: "Redigera Beskrivning",
        additional_actions_section_label: "ytterligare åtgärder",
        delete_task_tooltip: "Radera Uppgift",
        edit_tag_color_form_label: "Redigera Färg",
        edit_tag_color_button_label: "Redigera Färg",
        set_tag_color_button_label: "Sätt Färg",
        cancel_tag_color_update_label: "Avbryt Färguppdatering",
        edit_tag_name_button_label: "Redigera Namn",
        edit_tag_name_form_label: "Redigera Namn",
        set_tag_name_button_label: "Sätt Namn",
        cancel_tag_name_update_button_label: "Avbryt Namnändring",
        delete_tag_button_label: "Radera Tagg",
        archive_tag_button_label: "Arkivera Tagg",
        unarchive_tag_button_label: "Återställ Tagg",
        edit_user_color_form_label: "Redigera Färg",
        set_user_color_button_label: "Sätt Färg",
        cancel_user_color_update_button_label: "Avbryt Färguppdatering",
        edit_user_color_button_label: "Redigera Färg",
        edit_user_name_form_label: "Redigera Namn",
        set_user_name_button_label: "Sätt Namn",
        cancel_user_name_update_button_label: "Avbryt Namnändring",
        edit_user_name_button_label: "Redigera Namn",
        delete_user_button_label: "Radera Användare",
        task_status_section_label: "Uppgiftens Status",
        filters_section_label: "Filter",
        languages_section_title: "Språk",
        custom_task_button_label: "Anpassad Uppgift",
        board_list_section_label: "Tavellista",
        join_board_button_label: "Gå med i Tavla",
        create_new_board_button_label: "Skapa Ny Tavla",
        or_label: "eller",
        chat_gpt_limit_exceeded_title: "ChatGPT Begränsning Överskriden",
        chat_gpt_limit_exceeded_content: "Du har nått gränsen för ChatGPT-anrop. Försök igen senare.",
        chat_gpt_waiting_message: "Pratar med ChatGPT...",
        chat_gpt_error_title: "ChatGPT Fel",
        chat_gpt_error_content: "Ett fel inträffade vid försök att ansluta till ChatGPT. Försök igen senare.",
        chat_gpt_prompt_input_title: "ChatGPT Förfrågan",
        chat_gpt_daily_attempts_left: "dagliga försök kvar",
        chat_gpt_prompt_input_content: "eller välj en från förslagen nedan:",
        chat_gpt_prompt_input_form_label: "chat gpt förfrågan",
        chat_gpt_prompt_input_label: "Förfrågan:",
        suggest_cupcake_recipe_prompt: "föreslå recept på cupcakes",
        paint_bedroom_prompt: "måla sovrummet",
        friends_over_for_bbq_prompt: "vänner över för BBQ",
        prepare_for_rome_vacation_prompt: "förbered för Romsemester",
        house_tidy_prompt: "städa huset",
        fix_fence_prompt: "fixa staketet",
        join_board_form_label: "Gå med i Tavla",
        join_board_input_label: "Tavlans Namn",
        cancel_joining_board_button_label: "Avbryt anslutning till Tavla",
        add_task_button_label: "Lägg Till Uppgift",
        remove_board_button_label: "Ta Bort Tavla",
        new_task_form_label: "Ny Uppgift",
        cancel_adding_new_task_button_label: "Avbryt Tilllägg av Ny Uppgift",
        navigation_section_label: "Navigering",
        toggle_actions_drawer_button_label: "Växla Åtgärdslådan",
        toggle_show_filters_button_label: "Växla Visning av Filter",
        themes_section_label: "Teman",
        close_theme_selector_button_label: "Stäng Temaväljare",
        close_filters_button_label: "Stäng Filter",
        board_link: "Tavla",
        tags_link: "Taggar",
        users_link: "Användare",
        archive_link: "Arkiv",
    },
};
