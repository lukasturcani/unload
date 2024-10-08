use super::{Text, Translation};

pub const TR: Translation<&'static str> = Translation {
    id: "tr",
    name: "TR - Turkish",
    text: Text {
        to_do_column_title: "Yapılacaklar",
        in_progress_column_title: "Devam Ediyor",
        done_column_title: "Tamamlandı",
        pick_language_tooltip: "Dili Seç",
        toggle_show_themes_tooltip: "Temayı Değiştir",
        toggle_dense_view_tooltip: "Yoğun Görünümü Aç/Kapat",
        edit_board_title_tooltip: "Başlığı Düzenle",
        board_title_input_label: "Başlık",
        board_title_update_form_label: "Pano Başlığını Güncelle",
        set_board_title_button_label: "Başlığı Ayarla",
        cancel_board_title_update_button_label: "Başlık Güncellemesini İptal Et",
        task_title_input_label: "Başlık",
        edit_task_title_tooltip: "Başlığı Düzenle",
        task_title_update_form_label: "Görev Başlığını Güncelle",
        set_task_title_button_label: "Başlığı Ayarla",
        cancel_task_title_update_button_label: "Başlık Güncellemesini İptal Et",
        set_task_status_section_label: "Görev Durumunu Ayarla",
        to_do_button_tooltip: "Yapılacak",
        in_progress_button_tooltip: "Devam Ediyor",
        done_button_tooltip: "Tamamlandı",
        task_actions_section_label: "Görev İşlemleri",
        duplicate_task_button_tooltip: "Görevi Kopyala",
        archive_task_button_tooltip: "Görevi Arşivle",
        unarchive_task_button_tooltip: "Görevi Geri Yükle",
        assignees_section_label: "Görevliler",
        assign_user_toggle_button_tooltip: "Kullanıcıyı Ata",
        toggle_user_filter_button_label: "kullanıcı filtresini aç/kapat",
        assignee_selection_section_label: "görevli seçimi",
        add_user_button_label: "Kullanıcı Ekle",
        add_user_form_label: "Kullanıcı Ekle",
        user_name_input_label: "İsim",
        cancel_adding_new_user_button_label: "kullanıcı eklemeyi iptal et",
        remove_user_from_task_button_label: "görevden kullanıcıyı çıkar",
        tags_section_label: "Etiketler",
        tag_selection_section_label: "Etiket Seçimi",
        add_tag_button_label: "Etiket Ekle",
        add_tag_form_label: "Etiket Ekle",
        tag_name_input_label: "İsim",
        add_tag_toggle_button_tooltip: "Etiket Ekle",
        cancel_adding_new_tag_button_label: "etiket eklemeyi iptal et",
        toggle_tag_filter_button_label: "etiket filtresini aç/kapat",
        remove_tag_from_task_button_label: "etiketi görevden çıkar",
        toggle_expand_task_button_label: "görevi genişlet/daralt",
        due_date_section_label: "son tarih",
        edit_due_date_tooltip: "Son Tarihi Düzenle",
        due_date_form_label: "son tarihi ayarla",
        due_date_input_label: "Son Tarih",
        set_due_date_button_label: "son tarihi ayarla",
        cancel_due_date_update_button_label: "son tarih güncellemesini iptal et",
        color_picker_legend_label: "Renk",
        description_update_form_label: "açıklamayı güncelle",
        set_description_button_label: "açıklamayı ayarla",
        cancel_description_update_button_label: "açıklama güncellemesini iptal et",
        bullet_points_button_tooltip: "Madde İşaretleri",
        task_list_button_tooltip: "Görev Listesi",
        description_text_area_label: "Açıklama",
        description_section_label: "Açıklama",
        edit_description_tooltip: "Açıklamayı Düzenle",
        additional_actions_section_label: "ek işlemler",
        delete_task_tooltip: "Görevi Sil",
        edit_tag_color_form_label: "Rengi Düzenle",
        edit_tag_color_button_label: "Rengi Düzenle",
        set_tag_color_button_label: "Rengi Ayarla",
        cancel_tag_color_update_label: "Renk Güncellemesini İptal Et",
        edit_tag_name_button_label: "İsmi Düzenle",
        edit_tag_name_form_label: "İsmi Düzenle",
        set_tag_name_button_label: "İsmi Ayarla",
        cancel_tag_name_update_button_label: "İsim Güncellemesini İptal Et",
        delete_tag_button_label: "Etiketi Sil",
        archive_tag_button_label: "Eti kettle Arşivle",
        unarchive_tag_button_label: "Eti kettle Geri Yükle",
        edit_user_color_form_label: "Rengi Düzenle",
        set_user_color_button_label: "Rengi Ayarla",
        cancel_user_color_update_button_label: "Renk Güncellemesini İptal Et",
        edit_user_color_button_label: "Rengi Düzenle",
        edit_user_name_form_label: "İsmi Düzenle",
        set_user_name_button_label: "İsmi Ayarla",
        cancel_user_name_update_button_label: "İsim Güncellemesini İptal Et",
        edit_user_name_button_label: "İsmi Düzenle",
        delete_user_button_label: "Kullanıcıyı Sil",
        task_status_section_label: "Görev Durumu",
        filters_section_label: "Filtreler",
        languages_section_title: "Diller",
        custom_task_button_label: "Özel Görev",
        board_list_section_label: "Pano Listesi",
        join_board_button_label: "Panoya Katıl",
        create_new_board_button_label: "Yeni Pano Oluştur",
        or_label: "veya",
        chat_gpt_limit_exceeded_title: "ChatGPT Limiti Aşıldı",
        chat_gpt_limit_exceeded_content:
            "ChatGPT çağrılarında limite ulaştınız. Lütfen daha sonra tekrar deneyin.",
        chat_gpt_waiting_message: "ChatGPT ile konuşuluyor...",
        chat_gpt_error_title: "ChatGPT Hatası",
        chat_gpt_error_content:
            "ChatGPT'ye bağlanmaya çalışırken bir hata oluştu. Lütfen daha sonra tekrar deneyin.",
        chat_gpt_prompt_input_title: "ChatGPT Komutu",
        chat_gpt_daily_attempts_left: "günlük kalan deneme",
        chat_gpt_prompt_input_content: "veya aşağıdaki önerilerden birini seçin:",
        chat_gpt_prompt_input_form_label: "chat gpt komutu",
        chat_gpt_prompt_input_label: "Komut:",
        suggest_cupcake_recipe_prompt: "cupcake tarifi öner",
        paint_bedroom_prompt: "yatak odasını boya",
        friends_over_for_bbq_prompt: "barbekü için arkadaşlar gelsin",
        prepare_for_rome_vacation_prompt: "Roma tatili için hazırlanın",
        house_tidy_prompt: "evi toparla",
        fix_fence_prompt: "çiti tamir et",
        join_board_form_label: "Panoya Katıl",
        join_board_input_label: "Pano Adı",
        cancel_joining_board_button_label: "Panoya Katılmayı İptal Et",
        add_task_button_label: "Görev Ekle",
        remove_board_button_label: "Panoyu Kaldır",
        new_task_form_label: "Yeni Görev",
        cancel_adding_new_task_button_label: "Yeni Görev Eklemeyi İptal Et",
        navigation_section_label: "Gezinme",
        toggle_actions_drawer_button_label: "İşlemler Çekmecesini Aç/Kapat",
        toggle_show_filters_button_label: "Filtreleri Göster/Gizle",
        themes_section_label: "Temalar",
        close_theme_selector_button_label: "Tema Seçiciyi Kapat",
        close_filters_button_label: "Filtreleri Kapat",
        board_link: "Pano",
        tags_link: "Etiketler",
        users_link: "Kullanıcılar",
        archive_link: "Arşiv",
    },
};
