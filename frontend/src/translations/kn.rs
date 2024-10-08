use super::{Text, Translation};

pub const KN: Translation<&'static str> = Translation {
    id: "kn",
    name: "KN - ಕನ\u{ccd}ನಡ",
    text: Text {
        to_do_column_title: "ಮಾಡಬೇಕಾಗ\u{cbf}ದ\u{cc6}",
        in_progress_column_title: "ಆಗುತ\u{ccd}ತ\u{cbf}ದ\u{cc6}",
        done_column_title: "ಮುಗ\u{cbf}ಯ\u{cbf}ತು",
        pick_language_tooltip: "ಭಾಷ\u{cc6} ಆರ\u{cbf}ಸ\u{cbf}",
        toggle_show_themes_tooltip: "ಥೀಮ\u{ccd} ಬದಲಾಯ\u{cbf}ಸು",
        toggle_dense_view_tooltip: "ಸಾಂದ\u{ccd}ರ ದೃಷ\u{ccd}ಯ ಬದಲಾಯ\u{cbf}ಸು",
        edit_board_title_tooltip: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6} ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        board_title_input_label: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6}",
        board_title_update_form_label: "ಬೋರ\u{ccd}ಡ\u{ccd} ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6}ಯನ\u{ccd}ನು ನವೀಕರ\u{cbf}ಸ\u{cbf}",
        set_board_title_button_label: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6}ಯನ\u{ccd}ನು ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_board_title_update_button_label: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6} ನವೀಕರಣವನ\u{ccd}ನು ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        task_title_input_label: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6}",
        edit_task_title_tooltip: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6}ಯನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        task_title_update_form_label: "ಕಾರ\u{ccd}ಯದ ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6}ಯನ\u{ccd}ನು ನವೀಕರ\u{cbf}ಸ\u{cbf}",
        set_task_title_button_label: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6}ಯನ\u{ccd}ನು ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_task_title_update_button_label: "ಶೀರ\u{ccd}ಷ\u{cbf}ಕ\u{cc6} ನವೀಕರಣವನ\u{ccd}ನು ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        set_task_status_section_label: "ಕಾರ\u{ccd}ಯದ ಸ\u{ccd}ಥ\u{cbf}ತ\u{cbf}ಯನ\u{ccd}ನು ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        to_do_button_tooltip: "ಮಾಡಬೇಕಾಗ\u{cbf}ದ\u{cc6}",
        in_progress_button_tooltip: "ಆಗುತ\u{ccd}ತ\u{cbf}ದ\u{cc6}",
        done_button_tooltip: "ಮುಗ\u{cbf}ದ\u{cbf}ದ\u{cc6}",
        task_actions_section_label: "ಕಾರ\u{ccd}ಯಗಳ ಕ\u{ccd}ರ\u{cbf}ಯ\u{cc6}ಗಳು",
        duplicate_task_button_tooltip: "ದ\u{ccd}ವ\u{cbf}ರ\u{cc2}ಪಕಾರ\u{ccd}ಯ",
        archive_task_button_tooltip: "ಆರ\u{ccd}ಕೈವ\u{ccd} ಕಾರ\u{ccd}ಯ",
        unarchive_task_button_tooltip: "ಕಾರ\u{ccd}ಯವನ\u{ccd}ನು ಪುನಃಸ\u{ccd}ಥಾಪ\u{cbf}ಸು",
        assignees_section_label: "ಹಂಚ\u{cbf}ಕ\u{cc6}ದಾರರು",
        assign_user_toggle_button_tooltip: "ಬಳಕ\u{cc6}ದಾರರನ\u{ccd}ನು ಹ\u{cc2}ಕಂಪನ\u{cbf} ಮಾಡ\u{cbf}",
        toggle_user_filter_button_label: "ಬಳಕ\u{cc6}ದಾರರ ಫ\u{cbf}ಲ\u{ccd}ಟರ\u{ccd} ಬದಲಾಯ\u{cbf}ಸ\u{cbf}",
        assignee_selection_section_label: "ಹಂಚ\u{cbf}ಕ\u{cc6}ದಾರರ ಆಯ\u{ccd}ಕ\u{cc6}",
        add_user_button_label: "ಬಳಕ\u{cc6}ದಾರರನ\u{ccd}ನು ಸೇರ\u{cbf}ಸ\u{cbf}",
        add_user_form_label: "ಬಳಕ\u{cc6}ದಾರರನ\u{ccd}ನು ಸೇರ\u{cbf}ಸ\u{cbf}",
        user_name_input_label: "ಹ\u{cc6}ಸರು",
        cancel_adding_new_user_button_label: "ಬಳಕ\u{cc6}ದಾರರನ\u{ccd}ನು ಸೇರ\u{cbf}ಸುವುದನ\u{ccd}ನು ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        remove_user_from_task_button_label: "ಕಾರ\u{ccd}ಯದ\u{cbf}ಂದ ಬಳಕ\u{cc6}ದಾರರನ\u{ccd}ನು ಕಳಚ\u{cbf}",
        tags_section_label: "ಟ\u{ccd}ಯಾಗ\u{ccd}ಗಳು",
        tag_selection_section_label: "ಟ\u{ccd}ಯಾಗ\u{ccd} ಆಯ\u{ccd}ಕ\u{cc6}",
        add_tag_button_label: "ಟ\u{ccd}ಯಾಗ\u{ccd} ಸೇರ\u{cbf}ಸ\u{cbf}",
        add_tag_form_label: "ಟ\u{ccd}ಯಾಗ\u{ccd} ಸೇರ\u{cbf}ಸ\u{cbf}",
        tag_name_input_label: "ಹ\u{cc6}ಸರು",
        add_tag_toggle_button_tooltip: "ಟ\u{ccd}ಯಾಗ\u{ccd} ಸೇರ\u{cbf}ಸ\u{cbf}",
        cancel_adding_new_tag_button_label: "ಟ\u{ccd}ಯಾಗ\u{ccd} ಸೇರ\u{cbf}ಸುವುದನ\u{ccd}ನು ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        toggle_tag_filter_button_label: "ಟ\u{ccd}ಯಾಗ\u{ccd} ಫ\u{cbf}ಲ\u{ccd}ಟರ\u{ccd} ಬದಲಾಯ\u{cbf}ಸ\u{cbf}",
        remove_tag_from_task_button_label: "ಕಾರ\u{ccd}ಯದ\u{cbf}ಂದ ಟ\u{ccd}ಯಾಗ\u{ccd}ಗಳು ತ\u{cc6}ಗ\u{cc6}ಸ\u{cbf}",
        toggle_expand_task_button_label: "ಕಾರ\u{ccd}ಯವನ\u{ccd}ನು ವ\u{cbf}ಸ\u{ccd}ತರ\u{cbf}ಸ\u{cbf}",
        due_date_section_label: "ನ\u{cbf}ಗದ\u{cbf}ತ ದ\u{cbf}ನಾಂಕ",
        edit_due_date_tooltip: "ನ\u{cbf}ಗದ\u{cbf}ತ ದ\u{cbf}ನಾಂಕ ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        due_date_form_label: "ನ\u{cbf}ಗದ\u{cbf}ತ ದ\u{cbf}ನಾಂಕ ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        due_date_input_label: "ನ\u{cbf}ಗದ\u{cbf}ತ ದ\u{cbf}ನಾಂಕ",
        set_due_date_button_label: "ನ\u{cbf}ಗದ\u{cbf}ತ ದ\u{cbf}ನಾಂಕ ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_due_date_update_button_label: "ನ\u{cbf}ಗದ\u{cbf}ತ ದ\u{cbf}ನಾಂಕ ನವೀಕರಣ ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        color_picker_legend_label: "ಬಣ\u{ccd}ಣ",
        description_update_form_label: "ವ\u{cbf}ವರಣ\u{cc6} ನವೀಕರಣ",
        set_description_button_label: "ವ\u{cbf}ವರಣ\u{cc6} ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_description_update_button_label: "ವ\u{cbf}ವರಣ\u{cc6} ನವೀಕರಣ ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        bullet_points_button_tooltip: "ಬುಲ\u{cc6}ಟ\u{ccd} ಪಾಯ\u{cbf}ಂಟ\u{ccd}ಸ\u{ccd}",
        task_list_button_tooltip: "ಕಾರ\u{ccd}ಯಪಟ\u{ccd}ಟ\u{cbf}",
        description_text_area_label: "ವ\u{cbf}ವರಣ\u{cc6}",
        description_section_label: "ವ\u{cbf}ವರಣ\u{cc6}",
        edit_description_tooltip: "ವ\u{cbf}ವರಣ\u{cc6} ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        additional_actions_section_label: "ಹ\u{cc6}ಚ\u{ccd}ಚುವರ\u{cbf} ಕ\u{ccd}ರಮಗಳು",
        delete_task_tooltip: "ಕಾರ\u{ccd}ಯವನ\u{ccd}ನು ಅಳ\u{cbf}ಸ\u{cbf}",
        edit_tag_color_form_label: "ಬಣ\u{ccd}ಣವನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        edit_tag_color_button_label: "ಬಣ\u{ccd}ಣವನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        set_tag_color_button_label: "ಬಣ\u{ccd}ಣವನ\u{ccd}ನು ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_tag_color_update_label: "ಬಣ\u{ccd}ಣದ ನವೀಕರಣ ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        edit_tag_name_button_label: "ಹ\u{cc6}ಸರನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        edit_tag_name_form_label: "ಹ\u{cc6}ಸರನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        set_tag_name_button_label: "ಹ\u{cc6}ಸರನ\u{ccd}ನು ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_tag_name_update_button_label: "ಹ\u{cc6}ಸರ\u{cbf}ನ ನವೀಕರಣವನ\u{ccd}ನು ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        delete_tag_button_label: "ಟ\u{ccd}ಯಾಗನ\u{ccd}ನು ಅಳ\u{cbf}ಸು",
        archive_tag_button_label: "ಆರ\u{ccd}ಕೈವ\u{ccd} ಟ\u{ccd}ಯಾಗ\u{ccd}",
        unarchive_tag_button_label: "ಪುನಃಸ\u{ccd}ಥಾಪ\u{cbf}ಸು ಟ\u{ccd}ಯಾಗ\u{ccd}",
        edit_user_color_form_label: "ಬಣ\u{ccd}ಣವನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        set_user_color_button_label: "ಬಣ\u{ccd}ಣವನ\u{ccd}ನು ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_user_color_update_button_label: "ಬಣ\u{ccd}ಣದ ನವೀಕರಣವನ\u{ccd}ನು ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        edit_user_color_button_label: "ಬಣ\u{ccd}ಣವನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        edit_user_name_form_label: "ಹ\u{cc6}ಸರನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        set_user_name_button_label: "ಹ\u{cc6}ಸರನ\u{ccd}ನು ಸ\u{cc6}ಟ\u{ccd} ಮಾಡ\u{cbf}",
        cancel_user_name_update_button_label: "ಹ\u{cc6}ಸರ\u{cbf}ನ ನವೀಕರಣವನ\u{ccd}ನು ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        edit_user_name_button_label: "ಹ\u{cc6}ಸರನ\u{ccd}ನು ಸಂಪಾದ\u{cbf}ಸ\u{cbf}",
        delete_user_button_label: "ಬಳಕ\u{cc6}ದಾರರನ\u{ccd}ನು ಅಳ\u{cbf}ಸು",
        task_status_section_label: "ಕಾರ\u{ccd}ಯದ ಸ\u{ccd}ಥ\u{cbf}ತ\u{cbf}",
        filters_section_label: "ಫ\u{cbf}ಲ\u{ccd}ಟರ\u{ccd}\u{200c}ಗಳು",
        languages_section_title: "ಭಾಷ\u{cc6}ಗಳು",
        custom_task_button_label: "ಕಸ\u{ccd}ಟಮ\u{ccd} ಕಾರ\u{ccd}ಯ",
        board_list_section_label: "ಬೋರ\u{ccd}ಡ\u{ccd} ಪಟ\u{ccd}ಟ\u{cbf}",
        join_board_button_label: "ಬೋರ\u{ccd}ಡ\u{ccd} ಸೇರ\u{cbf}ರ\u{cbf}",
        create_new_board_button_label: "ಹೊಸ ಬೋರ\u{ccd}ಡ\u{ccd} ಸೃಷ\u{ccd}ಟ\u{cbf}ಸ\u{cbf}",
        or_label: "ಅಥವಾ",
        chat_gpt_limit_exceeded_title: "ಚಾಟ\u{ccd}\u{200c}ಜ\u{cbf}ಪ\u{cbf}ಟ\u{cbf} ಮ\u{cbf}ತ\u{cbf} ಮೀರ\u{cbf} ಹೋಗ\u{cbf}ದ\u{cc6}",
        chat_gpt_limit_exceeded_content: "ನೀವು ಚಾಟ\u{ccd}\u{200c}ಜ\u{cbf}ಪ\u{cbf}ಟ\u{cbf} ಕರ\u{cc6}ಗಳ ಬೃಹತ\u{ccd} ನಲ\u{ccd}ಲ\u{cbf} ತಲುಪ\u{cbf}ದ\u{ccd}ದೀರ\u{cbf}. ದಯವ\u{cbf}ಟ\u{ccd}ಟು ನಂತರ ಪುನಃ ಪ\u{ccd}ರಯತ\u{ccd}ನ\u{cbf}ಸ\u{cbf}.",
        chat_gpt_waiting_message: "ಚಾಟ\u{ccd}\u{200c}ಜ\u{cbf}ಪ\u{cbf}ಟ\u{cbf}ಗ\u{cc6} ಮಾತನಾಡುತ\u{ccd}ತ\u{cbf}ದ\u{cc6}...",
        chat_gpt_error_title: "ಚಾಟ\u{ccd}\u{200c}ಜ\u{cbf}ಪ\u{cbf}ಟ\u{cbf} ದೋಷ",
        chat_gpt_error_content: "ಚಾಟ\u{ccd} ಜ\u{cbf}ಪ\u{cbf}ಟ\u{cbf}ಗ\u{cc6} ಸಂಪರ\u{ccd}ಕ\u{cbf}ಸಲು ಪ\u{ccd}ರಯತ\u{ccd}ನ\u{cbf}ಸುವಾಗ ದೋಷ ಸಂಭವ\u{cbf}ಸ\u{cbf}ದ\u{cc6}. ದಯವ\u{cbf}ಟ\u{ccd}ಟು ನಂತರ ಪುನಃ ಪ\u{ccd}ರಯತ\u{ccd}ನ\u{cbf}ಸ\u{cbf}.",
        chat_gpt_prompt_input_title: "ಚಾಟ\u{ccd}\u{200c}ಜ\u{cbf}ಪ\u{cbf}ಟ\u{cbf} ಪ\u{ccd} ರಾಂಪ\u{ccd}ಟ\u{ccd}",
        chat_gpt_daily_attempts_left: "ದ\u{cbf}ನದ ಪ\u{ccd}ರಯತ\u{ccd}ನಗಳು ಉಳ\u{cbf}ದ\u{cbf}ವ\u{cc6}",
        chat_gpt_prompt_input_content: "ಅಥವಾ, ಕ\u{cc6}ಳಗ\u{cbf}ನ ಸ\u{cc2}ಚನ\u{cc6}ಗಳ\u{cbf}ಂದ ಒಂದನ\u{ccd}ನು ಆರ\u{cbf}ಸ\u{cbf}ರ\u{cbf}:",
        chat_gpt_prompt_input_form_label: "ಚಾಟ\u{ccd}\u{200c}ಜ\u{cbf}ಪ\u{cbf}ಟ\u{cbf} ಸ\u{cc2}ಚನ\u{cc6}",
        chat_gpt_prompt_input_label: "ಸ\u{cc2}ಚನ\u{cc6}:",
        suggest_cupcake_recipe_prompt: "ಕಪ\u{ccd}\u{200c}ಕೇಕ\u{ccd} ಕ\u{ccd}ವರೀತ\u{cbf} ಸ\u{cc2}ಚ\u{cbf}ಸ\u{cbf}ರ\u{cbf}",
        paint_bedroom_prompt: "ಶಾಯ\u{cbf}ತರ ಹಳ\u{cc6}ಯಗೊಳ\u{cbf}ಸ\u{cbf}ರ\u{cbf}",
        friends_over_for_bbq_prompt: "ಬ\u{cbf}ಬ\u{cbf}ಕ\u{ccd}ಯ\u{cc2}ಗಾಗ\u{cbf} ಸ\u{ccd}ನೇಹ\u{cbf}ತರ ಮನ\u{cc6}ಗ\u{cc6} ಬರಲು",
        prepare_for_rome_vacation_prompt: "ರೋಮ\u{ccd} ರಜ\u{cc6}ಗಾಗ\u{cbf} ತರಬೇತ\u{cbf}",
        house_tidy_prompt: "ಇಲವನ\u{ccd}ನು ತಯಾರ\u{cbf}ಸ\u{cbf}ರ\u{cbf}",
        fix_fence_prompt: "ಕಾಂಪ\u{cc6}ಯನ\u{ccd}ನು ಸರ\u{cbf}ಪಡ\u{cbf}ಸು",
        join_board_form_label: "ಬೋರ\u{ccd}ಡ\u{ccd} ಸೇರ\u{cbf}ರ\u{cbf}",
        join_board_input_label: "ಬೋರ\u{ccd}ಡ\u{ccd} ಹ\u{cc6}ಸರು",
        cancel_joining_board_button_label: "ಬೋರ\u{ccd}ಡ\u{ccd} ಸೇರು ನೀವೇನು",
        add_task_button_label: "ಕಾರ\u{ccd}ಯ ಸೇರ\u{cbf}ಸ\u{cbf}",
        remove_board_button_label: "ಬೋರ\u{ccd}ಡ\u{ccd} ತ\u{cc6}ಗ\u{cc6}ಸ\u{cbf}ರ\u{cbf}",
        new_task_form_label: "ಹೊಸ ಕಾರ\u{ccd}ಯ",
        cancel_adding_new_task_button_label: "ಹೊಸ ಕಾರ\u{ccd}ಯವನ\u{ccd}ನು ಸೇರ\u{cbf}ಸುವುದನ\u{ccd}ನ ರದ\u{ccd}ದುಮಾಡ\u{cbf}",
        navigation_section_label: "ಸಂಚಲನ",
        toggle_actions_drawer_button_label: "ಕ\u{ccd}ರ\u{cbf}ಯ\u{cc6}ಗಳ ಡ\u{ccd}ರೋರನ\u{ccd}ನು ಬದಲಾಯ\u{cbf}ಸ\u{cbf}",
        toggle_show_filters_button_label: "ಫ\u{cbf}ಲ\u{ccd}ಟರ\u{ccd}\u{200c}ಗಳನ\u{ccd}ನು ತೋರ\u{cbf}ಸಲು ಬದಲಾಯ\u{cbf}ಸ\u{cbf}",
        themes_section_label: "ಥೀಮ\u{ccd}ಗಳು",
        close_theme_selector_button_label: "ಥೀಮ\u{ccd}\u{200c}ಛಾಯಾಗೃತ\u{cbf} ಮುಚ\u{ccd}ಚ\u{cbf}",
        close_filters_button_label: "ಫ\u{cbf}ಲ\u{ccd}ಟರ\u{ccd}\u{200c}ಗಳನ\u{ccd}ನು ಮುಚ\u{ccd}ಚ\u{cbf}",
        board_link: "ಬೋರ\u{ccd}ಡ\u{ccd}",
        tags_link: "ಟ\u{ccd}ಯಾಗ\u{ccd}ಗಳು",
        users_link: "ಬಳಕ\u{cc6}ದಾರರು",
        archive_link: "ಆರ\u{ccd}ಕೈವ\u{ccd}",
    },
};
