use super::{Text, Translation};

pub const FA: Translation<&'static str> = Translation {
    id: "fa",
    name: "FA - Persian",
    text: Text {
        to_do_column_title: "برای انجام",
        in_progress_column_title: "در حال انجام",
        done_column_title: "انجام شده",
        pick_language_tooltip: "انتخاب زبان",
        toggle_show_themes_tooltip: "تغییر تم",
        toggle_dense_view_tooltip: "نمای فشرده",
        edit_board_title_tooltip: "ویرایش عنوان",
        board_title_input_label: "عنوان",
        board_title_update_form_label: "به روز رسانی عنوان تخته",
        set_board_title_button_label: "تنظیم عنوان",
        cancel_board_title_update_button_label: "لغو به روز رسانی عنوان",
        task_title_input_label: "عنوان",
        edit_task_title_tooltip: "ویرایش عنوان",
        task_title_update_form_label: "به روز رسانی عنوان وظیفه",
        set_task_title_button_label: "تنظیم عنوان",
        cancel_task_title_update_button_label: "لغو به روز رسانی عنوان وظیفه",
        set_task_status_section_label: "تنظیم وضعیت وظیفه",
        to_do_button_tooltip: "برای انجام",
        in_progress_button_tooltip: "در حال انجام",
        done_button_tooltip: "انجام شده",
        task_actions_section_label: "عملکردهای وظیفه",
        duplicate_task_button_tooltip: "کپی وظیفه",
        archive_task_button_tooltip: "آرشیو وظیفه",
        unarchive_task_button_tooltip: "بازیابی وظیفه",
        assignees_section_label: "موظفین",
        assign_user_toggle_button_tooltip: "تخصیص کاربر",
        toggle_user_filter_button_label: "فیلتر کاربر",
        assignee_selection_section_label: "انتخاب کاربر",
        add_user_button_label: "افزودن کاربر",
        add_user_form_label: "افزودن کاربر",
        user_name_input_label: "نام",
        cancel_adding_new_user_button_label: "لغو اضافه کردن کاربر",
        remove_user_from_task_button_label: "حذف کاربر از وظیفه",
        tags_section_label: "برچسب\u{200c}ها",
        tag_selection_section_label: "انتخاب برچسب",
        add_tag_button_label: "افزودن برچسب",
        add_tag_form_label: "افزودن برچسب",
        tag_name_input_label: "نام",
        add_tag_toggle_button_tooltip: "افزودن برچسب",
        cancel_adding_new_tag_button_label: "لغو اضافه کردن برچسب",
        toggle_tag_filter_button_label: "فیلتر برچسب",
        remove_tag_from_task_button_label: "حذف برچسب از وظیفه",
        toggle_expand_task_button_label: "نمایش گسترده وظیفه",
        due_date_section_label: "تاریخ سررسید",
        edit_due_date_tooltip: "ویرایش تاریخ سررسید",
        due_date_form_label: "تنظیم تاریخ سررسید",
        due_date_input_label: "تاریخ سررسید",
        set_due_date_button_label: "تنظیم تاریخ سررسید",
        cancel_due_date_update_button_label: "لغو به روز رسانی تاریخ سررسید",
        color_picker_legend_label: "رنگ",
        description_update_form_label: "به روز رسانی توضیح",
        set_description_button_label: "تنظیم توضیح",
        cancel_description_update_button_label: "لغو به روز رسانی توضیح",
        bullet_points_button_tooltip: "نقاط گلوله\u{200c}ای",
        task_list_button_tooltip: "لیست وظایف",
        description_text_area_label: "توضیح",
        description_section_label: "توضیح",
        edit_description_tooltip: "ویرایش توضیح",
        additional_actions_section_label: "عملکردهای اضافی",
        delete_task_tooltip: "حذف وظیفه",
        edit_tag_color_form_label: "ویرایش رنگ",
        edit_tag_color_button_label: "ویرایش رنگ",
        set_tag_color_button_label: "تنظیم رنگ",
        cancel_tag_color_update_label: "لغو به روز رسانی رنگ",
        edit_tag_name_button_label: "ویرایش نام",
        edit_tag_name_form_label: "ویرایش نام",
        set_tag_name_button_label: "تنظیم نام",
        cancel_tag_name_update_button_label: "لغو به روز رسانی نام",
        delete_tag_button_label: "حذف برچسب",
        archive_tag_button_label: "آرشیو برچسب",
        unarchive_tag_button_label: "بازیابی برچسب",
        edit_user_color_form_label: "ویرایش رنگ",
        set_user_color_button_label: "تنظیم رنگ",
        cancel_user_color_update_button_label: "لغو به روز رسانی رنگ",
        edit_user_color_button_label: "ویرایش رنگ",
        edit_user_name_form_label: "ویرایش نام",
        set_user_name_button_label: "تنظیم نام",
        cancel_user_name_update_button_label: "لغو به روز رسانی نام",
        edit_user_name_button_label: "ویرایش نام",
        delete_user_button_label: "حذف کاربر",
        task_status_section_label: "وضعیت وظیفه",
        filters_section_label: "فیلترها",
        languages_section_title: "زبان\u{200c}ها",
        custom_task_button_label: "وظیفه سفارشی",
        board_list_section_label: "لیست تخته\u{200c}ها",
        join_board_button_label: "پیوستن به تخته",
        create_new_board_button_label: "ایجاد تخته جدید",
        or_label: "یا",
        chat_gpt_limit_exceeded_title: "محدودیت ChatGPT به پایان رسید",
        chat_gpt_limit_exceeded_content: "شما به حد مجاز تماس با ChatGPT رسیده\u{200c}اید. لطفا\u{64b} بعدا\u{64b} دوباره امتحان کنید.",
        chat_gpt_waiting_message: "در حال گفتگو با ChatGPT...",
        chat_gpt_error_title: "خطای ChatGPT",
        chat_gpt_error_content: "خطایی در اتصال به ChatGPT رخ داده است. لطفا\u{64b} بعدا\u{64b} دوباره امتحان کنید.",
        chat_gpt_prompt_input_title: "درخواست ChatGPT",
        chat_gpt_daily_attempts_left: "تلاش\u{200c}های روزانه باقیمانده",
        chat_gpt_prompt_input_content: "یا یکی از پیشنهادات زیر را انتخاب کنید:",
        chat_gpt_prompt_input_form_label: "درخواست ChatGPT",
        chat_gpt_prompt_input_label: " درخواست:",
        suggest_cupcake_recipe_prompt: "پیشنهاد دستور پخت کیک فنجانی",
        paint_bedroom_prompt: "نقاشی اتاق خواب",
        friends_over_for_bbq_prompt: "دوستان برای BBQ",
        prepare_for_rome_vacation_prompt: "آماده شدن برای تعطیلات رم",
        house_tidy_prompt: "مرتب کردن خانه",
        fix_fence_prompt: "تعمیر حصار",
        join_board_form_label: "پیوستن به تخته",
        join_board_input_label: "نام تخته",
        cancel_joining_board_button_label: "لغو پیوستن به تخته",
        add_task_button_label: "افزودن وظیفه",
        remove_board_button_label: "حذف تخته",
        new_task_form_label: "وظیفه جدید",
        cancel_adding_new_task_button_label: "لغو افزودن وظیفه جدید",
        navigation_section_label: "ناوبری",
        toggle_actions_drawer_button_label: "فعال\u{200c}سازی کشوی عملکردها",
        toggle_show_filters_button_label: "نمایش فیلترها",
        themes_section_label: "تم\u{200c}ها",
        close_theme_selector_button_label: "بستن انتخابگر تم",
        close_filters_button_label: "بستن فیلترها",
        board_link: "تخته",
        tags_link: "برچسب\u{200c}ها",
        users_link: "کاربران",
        archive_link: "آرشیو",
    },
};
