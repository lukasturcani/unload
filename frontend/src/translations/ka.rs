use super::{Text, Translation};

pub const KA: Translation<&'static str> = Translation {
    id: "ka",
    name: "KA - ქართული",
    text: Text {
        to_do_column_title: "გასაკეთებელი",
        in_progress_column_title: "მიმდინარეობს",
        done_column_title: "დასრულებული",
        pick_language_tooltip: "ენის არჩევა",
        toggle_show_themes_tooltip: "თემის შეცვლა",
        toggle_dense_view_tooltip: "გამოჩენის შეცვლა",
        edit_board_title_tooltip: "სათაურის რედაქტირება",
        board_title_input_label: "სათაური",
        board_title_update_form_label: "დაფის სათაურის განახლება",
        set_board_title_button_label: "სათაურის მითითება",
        cancel_board_title_update_button_label: "სათაურის განახლების გაუქმება",
        task_title_input_label: "სათაური",
        edit_task_title_tooltip: "სათაურის რედაქტირება",
        task_title_update_form_label: "დავალების სათაურის განახლება",
        set_task_title_button_label: "სათაურის მითითება",
        cancel_task_title_update_button_label: "სათაურის განახლების გაუქმება",
        set_task_status_section_label: "დავალების სტატუსის მითითება",
        to_do_button_tooltip: "გასაკეთებელი",
        in_progress_button_tooltip: "მიმდინარეობს",
        done_button_tooltip: "დასრულებული",
        task_actions_section_label: "დავალების მოქმედებები",
        duplicate_task_button_tooltip: "დავალების დუბლირება",
        archive_task_button_tooltip: "დავალების არქივში გადატანა",
        unarchive_task_button_tooltip: "დავალების აღდგენა",
        assignees_section_label: "დამსაზღვრელები",
        assign_user_toggle_button_tooltip: "მომხმარებლის მინიჭება",
        toggle_user_filter_button_label: "მომხმარებლის ფილტრის გადართვა",
        assignee_selection_section_label: "დამსაზღვრელის არჩევანი",
        add_user_button_label: "მომხმარებლის დამატება",
        add_user_form_label: "მომხმარებლის დამატება",
        user_name_input_label: "სახელი",
        cancel_adding_new_user_button_label: "მომხმარებლის დამატების გაუქმება",
        remove_user_from_task_button_label: "მომხმარებლის ამოცილება დავალებიდან",
        tags_section_label: "ტეგები",
        tag_selection_section_label: "ტეგების არჩევანი",
        add_tag_button_label: "ტეგის დამატება",
        add_tag_form_label: "ტეგის დამატება",
        tag_name_input_label: "სახელი",
        add_tag_toggle_button_tooltip: "ტეგის დამატება",
        cancel_adding_new_tag_button_label: "ტეგის დამატების გაუქმება",
        toggle_tag_filter_button_label: "ტეგების ფილტრის გადართვა",
        remove_tag_from_task_button_label: "ტეგის ამოცილება დავალებიდან",
        toggle_expand_task_button_label: "დავალების გაფართოების გადართვა",
        due_date_section_label: "სრულების ვადა",
        edit_due_date_tooltip: "სრულების ვადის რედაქტირება",
        due_date_form_label: "სრულების ვადის მითითება",
        due_date_input_label: "სრულების ვადა",
        set_due_date_button_label: "სრულების ვადის მითითება",
        cancel_due_date_update_button_label: "სრულების ვადის განახლების გაუქმება",
        color_picker_legend_label: "ფერი",
        description_update_form_label: "აღწერის განახლება",
        set_description_button_label: "აღწერის მითითება",
        cancel_description_update_button_label: "აღწერის განახლების გაუქმება",
        bullet_points_button_tooltip: "ტყვიების პუნქტები",
        task_list_button_tooltip: "დავალებების სია",
        description_text_area_label: "აღწერა",
        description_section_label: "აღწერა",
        edit_description_tooltip: "აღწერის რედაქტირება",
        additional_actions_section_label: "დამატებითი მოქმედებები",
        delete_task_tooltip: "დავალების წაშლა",
        edit_tag_color_form_label: "ფერის რედაქტირება",
        edit_tag_color_button_label: "ფერის რედაქტირება",
        set_tag_color_button_label: "ფერის მითითება",
        cancel_tag_color_update_label: "ფერის განახლების გაუქმება",
        edit_tag_name_button_label: "სახელის რედაქტირება",
        edit_tag_name_form_label: "სახელის რედაქტირება",
        set_tag_name_button_label: "სახელის მითითება",
        cancel_tag_name_update_button_label: "სახელის განახლების გაუქმება",
        delete_tag_button_label: "ტეგის წაშლა",
        archive_tag_button_label: "ტეგის არქივში გადატანა",
        unarchive_tag_button_label: "ტეგის აღდგენა",
        edit_user_color_form_label: "ფერის რედაქტირება",
        set_user_color_button_label: "ფერის მითითება",
        cancel_user_color_update_button_label: "ფერის განახლების გაუქმება",
        edit_user_color_button_label: "ფერის რედაქტირება",
        edit_user_name_form_label: "სახელის რედაქტირება",
        set_user_name_button_label: "სახელის მითითება",
        cancel_user_name_update_button_label: "სახელის განახლების გაუქმება",
        edit_user_name_button_label: "სახელის რედაქტირება",
        delete_user_button_label: "მომხმარებლის წაშლა",
        task_status_section_label: "დავალების სტატუსი",
        filters_section_label: "ფილტრები",
        languages_section_title: "ენები",
        custom_task_button_label: "კეთილმოწყობა",
        board_list_section_label: "დაფების სია",
        join_board_button_label: "დაფაზე შეერთება",
        create_new_board_button_label: "ახალი დაფის შექმნა",
        or_label: "ან",
        chat_gpt_limit_exceeded_title: "ChatGPT საზღვარი გადაჭარბებულია",
        chat_gpt_limit_exceeded_content: "თქვენ გადაჭარბეთ ChatGPT-მოხმარების საზღვარს. გთხოვთ, სცადეთ მოგვიანებით.",
        chat_gpt_waiting_message: "ChatGPT-თან საუბარი მიმდინარეობს...",
        chat_gpt_error_title: "ChatGPT შეცდომა",
        chat_gpt_error_content: "შეცდომა მოხდა, როცა ცდილობდით Chat GPT-სთან დაკავშირებას. სცადეთ მოგვიანებით.",
        chat_gpt_prompt_input_title: "ChatGPT მოთხოვნა",
        chat_gpt_daily_attempts_left: "დღიური მცდელობები დარჩენილია",
        chat_gpt_prompt_input_content: "ან აირჩიეთ წინადადება ქვემოთ ჩამოთვლილიდან:",
        chat_gpt_prompt_input_form_label: "chat gpt მოთხოვნა",
        chat_gpt_prompt_input_label: "მოთხოვნა:",
        suggest_cupcake_recipe_prompt: "კაპკეიკის რეცეპტის შეთავაზება",
        paint_bedroom_prompt: "საძინებლის შეღებვა",
        friends_over_for_bbq_prompt: "მეგობრების მიწვევა BBQ-ზე",
        prepare_for_rome_vacation_prompt: "რომის დასვენებისთვის მზადება",
        house_tidy_prompt: "სახლის მორბენა",
        fix_fence_prompt: "ღობის შეკეთება",
        join_board_form_label: "დაფაზე შეერთება",
        join_board_input_label: "დაფის სახელი",
        cancel_joining_board_button_label: "დაფაზე შეერთების გაუქმება",
        add_task_button_label: "დავალების დამატება",
        remove_board_button_label: "დაფის წაშლა",
        new_task_form_label: "ახალი დავალება",
        cancel_adding_new_task_button_label: "ახალი დავალების დამატების გაუქმება",
        navigation_section_label: "ნავიგაცია",
        toggle_actions_drawer_button_label: "მოქმედებების უჯრის გადართვა",
        toggle_show_filters_button_label: "ფილტრების ჩვენების გადართვა",
        themes_section_label: "თემები",
        close_theme_selector_button_label: "თემების გამორჩევის დახურვა",
        close_filters_button_label: "ფილტრების დახურვა",
        board_link: "დაფა",
        tags_link: "ტეგები",
        users_link: "მომხმარებლები",
        archive_link: "არქივი",
    },
};
