use super::{Text, Translation};

pub const JV: Translation<&'static str> = Translation {
    id: "jv",
    name: "JV - Javanese",
    text: Text {
        to_do_column_title: "Kudu Dilakoni",
        in_progress_column_title: "Sedang Dilakoni",
        done_column_title: "Rampung",
        pick_language_tooltip: "Pilih Basa",
        toggle_show_themes_tooltip: "Ganti Tema",
        toggle_dense_view_tooltip: "Ganti Tampilane Padhet",
        edit_board_title_tooltip: "Sunting Judul",
        board_title_input_label: "Judul",
        board_title_update_form_label: "Nganyarke Judul Papan",
        set_board_title_button_label: "Setel Judul",
        cancel_board_title_update_button_label: "Batal Nganyarke Judul",
        task_title_input_label: "Judul",
        edit_task_title_tooltip: "Sunting Judul",
        task_title_update_form_label: "Nganyarke Judul Tugas",
        set_task_title_button_label: "Setel Judul",
        cancel_task_title_update_button_label: "Batal Nganyarke Judul",
        set_task_status_section_label: "Setel Status Tugas",
        to_do_button_tooltip: "Kudu Dilakoni",
        in_progress_button_tooltip: "Sedang Dilakoni",
        done_button_tooltip: "Rampung",
        task_actions_section_label: "Lakuan Tugas",
        duplicate_task_button_tooltip: "Gandha Tugas",
        archive_task_button_tooltip: "Arsip Tugas",
        unarchive_task_button_tooltip: "Balekake Tugas",
        assignees_section_label: "Penggawe",
        assign_user_toggle_button_tooltip: "Tugas Pengguna",
        toggle_user_filter_button_label: "ganti saringan pengguna",
        assignee_selection_section_label: "seleksi penggawe",
        add_user_button_label: "Tambah Pengguna",
        add_user_form_label: "Tambah Pengguna",
        user_name_input_label: "Jeneng",
        cancel_adding_new_user_button_label: "batal nambahake pengguna",
        remove_user_from_task_button_label: "copot pengguna saka tugas",
        tags_section_label: "Tandha",
        tag_selection_section_label: "Pamilihan Tandha",
        add_tag_button_label: "Tambah Tandha",
        add_tag_form_label: "Tambah Tandha",
        tag_name_input_label: "Jeneng",
        add_tag_toggle_button_tooltip: "Tambah Tandha",
        cancel_adding_new_tag_button_label: "batal nambahake tandha",
        toggle_tag_filter_button_label: "ganti saringan tandha",
        remove_tag_from_task_button_label: "copot tandha saka tugas",
        toggle_expand_task_button_label: "ganti tampilan tugas",
        due_date_section_label: "tenggat wektu",
        edit_due_date_tooltip: "Sunting Tenggat Wektu",
        due_date_form_label: "Setel tenggat wektu",
        due_date_input_label: "Tenggat",
        set_due_date_button_label: "Setel tenggat",
        cancel_due_date_update_button_label: "batal nganyarke tenggat",
        color_picker_legend_label: "Warna",
        description_update_form_label: "nganyarke deskripsi",
        set_description_button_label: "setel deskripsi",
        cancel_description_update_button_label: "batal nganyarke deskripsi",
        bullet_points_button_tooltip: "Titik Poin",
        task_list_button_tooltip: "Dhaftar Tugas",
        description_text_area_label: "Deskripsi",
        description_section_label: "Deskripsi",
        edit_description_tooltip: "Sunting Deskripsi",
        additional_actions_section_label: "lakuan tambahan",
        delete_task_tooltip: "Busak Tugas",
        edit_tag_color_form_label: "Sunting Warna",
        edit_tag_color_button_label: "Sunting Warna",
        set_tag_color_button_label: "Setel Warna",
        cancel_tag_color_update_label: "Batal Nganyarke Warna",
        edit_tag_name_button_label: "Sunting Jeneng",
        edit_tag_name_form_label: "Sunting Jeneng",
        set_tag_name_button_label: "Setel Jeneng",
        cancel_tag_name_update_button_label: "Batal Nganyarke Jeneng",
        delete_tag_button_label: "Busak Tandha",
        archive_tag_button_label: "Arsip Tandha",
        unarchive_tag_button_label: "Balekake Tandha",
        edit_user_color_form_label: "Sunting Warna",
        set_user_color_button_label: "Setel Warna",
        cancel_user_color_update_button_label: "Batal Nganyarke Warna",
        edit_user_color_button_label: "Sunting Warna",
        edit_user_name_form_label: "Sunting Jeneng",
        set_user_name_button_label: "Setel Jeneng",
        cancel_user_name_update_button_label: "Batal Nganyarke Jeneng",
        edit_user_name_button_label: "Sunting Jeneng",
        delete_user_button_label: "Busak Pengguna",
        task_status_section_label: "Status Tugas",
        filters_section_label: "Saringan",
        languages_section_title: "Basa",
        custom_task_button_label: "Tugas Kustom",
        board_list_section_label: "Dhaftar Papan",
        join_board_button_label: "Gabung Papan",
        create_new_board_button_label: "Gawe Papan Anyar",
        or_label: "utawa",
        chat_gpt_limit_exceeded_title: "Watesan ChatGPT Kasuwun",
        chat_gpt_limit_exceeded_content:
            "Sampeyan wis tekan watesan panggilan ChatGPT. Monggo nyoba maneh mengko.",
        chat_gpt_waiting_message: "Ngomong karo ChatGPT...",
        chat_gpt_error_title: "Kesalahan ChatGPT",
        chat_gpt_error_content:
            "Terjadi kesalahan nalika nyoba nyambung karo Chat GPT. Monggo nyoba maneh mengko.",
        chat_gpt_prompt_input_title: "ChatGPT Pitakon",
        chat_gpt_daily_attempts_left: "upaya saben dina sing kasedhiya",
        chat_gpt_prompt_input_content: "utawa pilih salah sawijining saran ing ngisor iki:",
        chat_gpt_prompt_input_form_label: "pitakon chat gpt",
        chat_gpt_prompt_input_label: "Pitakon:",
        suggest_cupcake_recipe_prompt: "nyarankeake resep cupcake",
        paint_bedroom_prompt: "cat kamar turu",
        friends_over_for_bbq_prompt: "konco teka kanggo BBQ",
        prepare_for_rome_vacation_prompt: "nyiyapake liburan menyang Roma",
        house_tidy_prompt: "beresin omah",
        fix_fence_prompt: "betulin pager",
        join_board_form_label: "Gabung Papan",
        join_board_input_label: "Jeneng Papan",
        cancel_joining_board_button_label: "Batal Gabung Papan",
        add_task_button_label: "Tambah Tugas",
        remove_board_button_label: "Busak Papan",
        new_task_form_label: "Tugas Anyar",
        cancel_adding_new_task_button_label: "Batal Nambahake Tugas Anyar",
        navigation_section_label: "Navigasi",
        toggle_actions_drawer_button_label: "Ganti Laci Aksi",
        toggle_show_filters_button_label: "Ganti Tampilane Saringan",
        themes_section_label: "Tema",
        close_theme_selector_button_label: "Tutup Milih Tema",
        close_filters_button_label: "Tutup Saringan",
        board_link: "Papan",
        tags_link: "Tandha",
        users_link: "Pengguna",
        archive_link: "Arsip",
    },
};
