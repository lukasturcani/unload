use super::{Text, Translation};

pub const AR: Translation<&'static str> = Translation {
    id: "ar",
    name: "AR - العربية",
    text: Text {
        to_do_column_title: "لإنجاز",
        in_progress_column_title: "قيد التقدم",
        done_column_title: "تم",
        pick_language_tooltip: "اختر اللغة",
        toggle_show_themes_tooltip: "تغيير الثيم",
        toggle_dense_view_tooltip: "تبديل العرض المضغوط",
        edit_board_title_tooltip: "تحرير العنوان",
        board_title_input_label: "العنوان",
        board_title_update_form_label: "تحديث عنوان اللوحة",
        set_board_title_button_label: "تعيين العنوان",
        cancel_board_title_update_button_label: "إلغاء تحديث العنوان",
        task_title_input_label: "العنوان",
        edit_task_title_tooltip: "تحرير العنوان",
        task_title_update_form_label: "تحديث عنوان المهمة",
        set_task_title_button_label: "تعيين العنوان",
        cancel_task_title_update_button_label: "إلغاء تحديث العنوان",
        set_task_status_section_label: "تعيين حالة المهمة",
        to_do_button_tooltip: "لإنجاز",
        in_progress_button_tooltip: "قيد التقدم",
        done_button_tooltip: "تم",
        task_actions_section_label: "إجراءات المهمة",
        duplicate_task_button_tooltip: "مضاعفة المهمة",
        archive_task_button_tooltip: "أرشفة المهمة",
        unarchive_task_button_tooltip: "استرجاع المهمة",
        assignees_section_label: "المكلفون",
        assign_user_toggle_button_tooltip: "تعيين مستخدم",
        toggle_user_filter_button_label: "تبديل فلتر المستخدم",
        assignee_selection_section_label: "اختيار المكلف",
        add_user_button_label: "إضافة مستخدم",
        add_user_form_label: "إضافة مستخدم",
        user_name_input_label: "الاسم",
        cancel_adding_new_user_button_label: "إلغاء إضافة المستخدم",
        remove_user_from_task_button_label: "إزالة المستخدم من المهمة",
        tags_section_label: "الوسوم",
        tag_selection_section_label: "اختيار الوسام",
        add_tag_button_label: "إضافة وسام",
        add_tag_form_label: "إضافة وسام",
        tag_name_input_label: "الاسم",
        add_tag_toggle_button_tooltip: "إضافة وسام",
        cancel_adding_new_tag_button_label: "إلغاء إضافة الوسام",
        toggle_tag_filter_button_label: "تبديل فلتر الوسام",
        remove_tag_from_task_button_label: "إزالة الوسام من المهمة",
        toggle_expand_task_button_label: "تبديل توسيع المهمة",
        due_date_section_label: "تاريخ الاستحقاق",
        edit_due_date_tooltip: "تحرير تاريخ الاستحقاق",
        due_date_form_label: "تعيين تاريخ الاستحقاق",
        due_date_input_label: "مستحق",
        set_due_date_button_label: "تعيين تاريخ الاستحقاق",
        cancel_due_date_update_button_label: "إلغاء تحديث تاريخ الاستحقاق",
        color_picker_legend_label: "اللون",
        description_update_form_label: "تحديث الوصف",
        set_description_button_label: "تعيين الوصف",
        cancel_description_update_button_label: "إلغاء تحديث الوصف",
        bullet_points_button_tooltip: "نقاط نقطية",
        task_list_button_tooltip: "قائمة المهام",
        description_text_area_label: "الوصف",
        description_section_label: "الوصف",
        edit_description_tooltip: "تحرير الوصف",
        additional_actions_section_label: "إجراءات إضافية",
        delete_task_tooltip: "حذف المهمة",
        edit_tag_color_form_label: "تحرير اللون",
        edit_tag_color_button_label: "تحرير اللون",
        set_tag_color_button_label: "تعيين اللون",
        cancel_tag_color_update_label: "إلغاء تحديث اللون",
        edit_tag_name_button_label: "تحرير الاسم",
        edit_tag_name_form_label: "تحرير الاسم",
        set_tag_name_button_label: "تعيين الاسم",
        cancel_tag_name_update_button_label: "إلغاء تحديث الاسم",
        delete_tag_button_label: "حذف الوسام",
        archive_tag_button_label: "أرشفة الوسام",
        unarchive_tag_button_label: "استرجاع الوسام",
        edit_user_color_form_label: "تحرير اللون",
        set_user_color_button_label: "تعيين اللون",
        cancel_user_color_update_button_label: "إلغاء تحديث اللون",
        edit_user_color_button_label: "تحرير اللون",
        edit_user_name_form_label: "تحرير الاسم",
        set_user_name_button_label: "تعيين الاسم",
        cancel_user_name_update_button_label: "إلغاء تحديث الاسم",
        edit_user_name_button_label: "تحرير الاسم",
        delete_user_button_label: "حذف المستخدم",
        task_status_section_label: "حالة المهمة",
        filters_section_label: "الفلاتر",
        languages_section_title: "اللغات",
        custom_task_button_label: "مهمة مخصصة",
        board_list_section_label: "قائمة اللوحات",
        join_board_button_label: "الانضمام إلى اللوحة",
        create_new_board_button_label: "إنشاء لوحة جديدة",
        or_label: "أو",
        chat_gpt_limit_exceeded_title: "تم تجاوز حد ChatGPT",
        chat_gpt_limit_exceeded_content:
            "لقد وصلت إلى الحد الأقصى من مكالمات ChatGPT. يرجى المحاولة مرة أخرى لاحق\u{64b}ا.",
        chat_gpt_waiting_message: "التحدث إلى ChatGPT...",
        chat_gpt_error_title: "خطأ ChatGPT",
        chat_gpt_error_content:
            "حدث خطأ أثناء محاولة الاتصال بـ ChatGPT. يرجى المحاولة مرة أخرى لاحق\u{64b}ا.",
        chat_gpt_prompt_input_title: "موجه ChatGPT",
        chat_gpt_daily_attempts_left: "المحاولات اليومية المتبقية",
        chat_gpt_prompt_input_content: "أو اختر واحد\u{64b}ا من الاقتراحات أدناه:",
        chat_gpt_prompt_input_form_label: "موجه ChatGPT",
        chat_gpt_prompt_input_label: "موجه:",
        suggest_cupcake_recipe_prompt: "اقتراح وصفة كب كيك",
        paint_bedroom_prompt: "طلاء غرفة النوم",
        friends_over_for_bbq_prompt: "الأصدقاء لتناول الشواء",
        prepare_for_rome_vacation_prompt: "التحضير لعطلة في روما",
        house_tidy_prompt: "تنظيف المنزل",
        fix_fence_prompt: "إصلاح السور",
        join_board_form_label: "الانضمام إلى اللوحة",
        join_board_input_label: "اسم اللوحة",
        cancel_joining_board_button_label: "إلغاء الانضمام إلى اللوحة",
        add_task_button_label: "إضافة مهمة",
        remove_board_button_label: "إزالة اللوحة",
        new_task_form_label: "مهمة جديدة",
        cancel_adding_new_task_button_label: "إلغاء إضافة المهمة الجديدة",
        navigation_section_label: "التنقل",
        toggle_actions_drawer_button_label: "تبديل درج الإجراءات",
        toggle_show_filters_button_label: "تبديل عرض الفلاتر",
        themes_section_label: "الثيمات",
        close_theme_selector_button_label: "إغلاق محدد الثيم",
        close_filters_button_label: "إغلاق الفلاتر",
        board_link: "اللوحة",
        tags_link: "الوسوم",
        users_link: "المستخدمون",
        archive_link: "الأرشيف",
    },
};
