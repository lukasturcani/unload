use super::{Text, Translation};

pub const AM: Translation<&'static str> = Translation {
    id: "am",
    name: "አማርኛ - Amharic",
    text: Text {
        to_do_column_title: "ለማድረግ",
        in_progress_column_title: "በመሥራት ላይ",
        done_column_title: "ተከናውኗል",
        pick_language_tooltip: "ቋንቋ ይምረጡ",
        toggle_show_themes_tooltip: "ገጽታ ይቀይሩ",
        toggle_dense_view_tooltip: "ጠቅላላ እይታ ይቀይሩ",
        edit_board_title_tooltip: "ርዕስን ያርትዑ",
        board_title_input_label: "ርዕስ",
        board_title_update_form_label: "የቦርድ ርዕስ አዘምን",
        set_board_title_button_label: "ርዕስ ያዘጋጁ",
        cancel_board_title_update_button_label: "የርዕስ አዘምንትን ሰርዝ",
        task_title_input_label: "ርዕስ",
        edit_task_title_tooltip: "ርዕስን ያርትዑ",
        task_title_update_form_label: "የተግባር ርዕስ አዘምን",
        set_task_title_button_label: "ርዕስ ያዘጋጁ",
        cancel_task_title_update_button_label: "የርዕስ አዘምንትን ሰርዝ",
        set_task_status_section_label: "የተግባር ሁኔታ ያዝዙ",
        to_do_button_tooltip: "ለማድረግ",
        in_progress_button_tooltip: "በመሥራት ላይ",
        done_button_tooltip: "ተከናውኗል",
        task_actions_section_label: "የተግባር እርምጃዎች",
        duplicate_task_button_tooltip: "ተግባርን አባል",
        archive_task_button_tooltip: "ተግባርን አሰቅብ",
        unarchive_task_button_tooltip: "ተግባርን እንደገና አቋርጥ",
        assignees_section_label: "መድርያ ጋር የወሰነ",
        assign_user_toggle_button_tooltip: "ተጠቃሚን ያዝዙ",
        toggle_user_filter_button_label: "የተጠቃሚ ማጣሪያ ይቀይሩ",
        assignee_selection_section_label: "ተወስነ አማራጭ",
        add_user_button_label: "ተጠቃሚን አክል",
        add_user_form_label: "ተጠቃሚን አክል",
        user_name_input_label: "ስም",
        cancel_adding_new_user_button_label: "ወደቀበተበት መያዝ ሰርዝ",
        remove_user_from_task_button_label: "ተጠቃሚን ከተግባር ሰርዝ",
        tags_section_label: "መለያዎች",
        tag_selection_section_label: "መለያ ምርጫ",
        add_tag_button_label: "መለያ አክል",
        add_tag_form_label: "መለያ አክል",
        tag_name_input_label: "ስም",
        add_tag_toggle_button_tooltip: "መለያ አክል",
        cancel_adding_new_tag_button_label: "መለያ አልባ መጨመር ሰርዝ",
        toggle_tag_filter_button_label: "የመለያ ማጣሪያ ይቀይሩ",
        remove_tag_from_task_button_label: "መለያ አስወግድ",
        toggle_expand_task_button_label: "ተግባርን ይስፋቡት",
        due_date_section_label: "እንቅስቃሴ ቀን",
        edit_due_date_tooltip: "እንቅስቃሴ ቀን ያርትዑ",
        due_date_form_label: "ቀን ያዝዙ",
        due_date_input_label: "እንቅስቃሴ በሞባይል",
        set_due_date_button_label: "ቀን ያዝዙ",
        cancel_due_date_update_button_label: "ቀን ማዘምንት ሰርዝ",
        color_picker_legend_label: "ቀለም",
        description_update_form_label: "መግለጫ አዘምን",
        set_description_button_label: "መግለጫ ያዘጋጁ",
        cancel_description_update_button_label: "መግለጫ አዘምንት ሰርዝ",
        bullet_points_button_tooltip: "ነጥብ ነጥቦች",
        task_list_button_tooltip: "የተግባር ዝርዝር",
        description_text_area_label: "መግለጫ",
        description_section_label: "መግለጫ",
        edit_description_tooltip: "መግለጫ ያርትዑ",
        additional_actions_section_label: "ተጨማሪ እርምጃዎች",
        delete_task_tooltip: "ተግባር ይሰርዙ",
        edit_tag_color_form_label: "ቀለም ያርትዑ",
        edit_tag_color_button_label: "ቀለም ያርትዑ",
        set_tag_color_button_label: "ቀለም ያዘጋጁ",
        cancel_tag_color_update_label: "የቀለም አዘምንት ሰርዝ",
        edit_tag_name_button_label: "ስም ያርትዑ",
        edit_tag_name_form_label: "ስም ያርትዑ",
        set_tag_name_button_label: "ስም ያዘጋጁ",
        cancel_tag_name_update_button_label: "የስም አዘምንት ሰርዝ",
        delete_tag_button_label: "መለያ ያጥፉ",
        archive_tag_button_label: "መለያ አሰቅብ",
        unarchive_tag_button_label: "መለያን እንደገና አቋርጥ",
        edit_user_color_form_label: "ቀለም ያርትዑ",
        set_user_color_button_label: "ቀለም ያዘጋጁ",
        cancel_user_color_update_button_label: "ቀለም አዘምንት ሰርዝ",
        edit_user_color_button_label: "ቀለም ያርትዑ",
        edit_user_name_form_label: "ስም ያርትዑ",
        set_user_name_button_label: "ስም ያዘጋጁ",
        cancel_user_name_update_button_label: "ስም አዘምንት ሰርዝ",
        edit_user_name_button_label: "ስም ያርትዑ",
        delete_user_button_label: "ተጠቃሚን ያጥፉ",
        task_status_section_label: "የተግባር ሁኔታ",
        filters_section_label: "ማጣሪያዎች",
        languages_section_title: "ቋንቋዎች",
        custom_task_button_label: "በተለይ የተሠራ ተግባር",
        board_list_section_label: "ቦርዶች ዝርዝር",
        join_board_button_label: "ቦርድ ይቀላቀሉ",
        create_new_board_button_label: "አዲስ ቦርድ ይፍጠሩ",
        or_label: "ወይም",
        chat_gpt_limit_exceeded_title: "የChatGPT ገደብን አለፉ",
        chat_gpt_limit_exceeded_content: "የChatGPT ጥሪዎችን ገደብ አልፈዋል። እባኮትን በኋላ እንደገና ይሞክሩ።",
        chat_gpt_waiting_message: "ከChatGPT ጋር እናገኛለን...",
        chat_gpt_error_title: "እባክዎን ChatGPT ከግል ንብረት ጋር አልተገናኝም",
        chat_gpt_error_content: "ChatGPT እንዲገናኝ አድርጎ ግፊት ሳይሆን ሊቀበል እንደማይችል ሁኔታ ተፈጥሮአል።",
        chat_gpt_prompt_input_title: "ChatGPT የምልክት ጥያቄ",
        chat_gpt_daily_attempts_left: "በዛሬ ቀን ቀሪ ውሳኔዎች",
        chat_gpt_prompt_input_content: "ወይም ከከታች ምርጫዎች ወንም ትምህርት ይጠቀሙ:",
        chat_gpt_prompt_input_form_label: "chat gpt ጥያቄ",
        chat_gpt_prompt_input_label: "ጥያቄ:",
        suggest_cupcake_recipe_prompt: "እንክኖቅግ ምግብፋዮ ይጠቀሙ",
        paint_bedroom_prompt: "ኊኸዳ ራ/balang",
        friends_over_for_bbq_prompt: "እንዲህን ባህጙትለውን",
        prepare_for_rome_vacation_prompt: "ለሩም እረፍት ይዘጋጁ",
        house_tidy_prompt: "ቤት አንካቻ",
        fix_fence_prompt: "እንግዳኞችን አቋርጥ",
        join_board_form_label: "ቦርድ ይቀላቀሉ",
        join_board_input_label: "የቦርድ ስም",
        cancel_joining_board_button_label: "ቦርድን አስተላለፍ",
        add_task_button_label: "ተግባር ያክሉ",
        remove_board_button_label: "ቦርድ ያስወግዱ",
        new_task_form_label: "አዲስ ተግባር",
        cancel_adding_new_task_button_label: "አዲስ ተግባር ማስተላለፍ ላይ",
        navigation_section_label: "አቅጣጫ",
        toggle_actions_drawer_button_label: "እርምጃዎችን ያስተካክሉ",
        toggle_show_filters_button_label: "የማጣሪያ እና ภ\u{e31}สษาት ቀሜ ሳይሆን",
        themes_section_label: "ገጽታዎች",
        close_theme_selector_button_label: "ገጽታ መርጫ ይዘጋኑ",
        close_filters_button_label: "ማጣሪያ ይዘጋኑ",
        board_link: "ቦርድ",
        tags_link: "መለያዎች",
        users_link: "ተጠቃሚዎች",
        archive_link: "ወቅታታት",
    },
};
