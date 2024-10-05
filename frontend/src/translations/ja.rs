use super::{Text, Translation};

pub const JA: Translation<&'static str> = Translation {
    id: "ja",
    name: "JA - Japanese",
    text: Text {
        to_do_column_title: "やること",
        in_progress_column_title: "進行中",
        done_column_title: "完了",
        pick_language_tooltip: "言語を選択",
        toggle_show_themes_tooltip: "テーマを変更",
        toggle_dense_view_tooltip: "密集ビューを切り替え",
        edit_board_title_tooltip: "タイトルを編集",
        board_title_input_label: "タイトル",
        board_title_update_form_label: "ボードタイトルを更新",
        set_board_title_button_label: "タイトルを設定",
        cancel_board_title_update_button_label: "タイトル更新をキャンセル",
        task_title_input_label: "タイトル",
        edit_task_title_tooltip: "タイトルを編集",
        task_title_update_form_label: "タスクタイトルを更新",
        set_task_title_button_label: "タイトルを設定",
        cancel_task_title_update_button_label: "タイトル更新をキャンセル",
        set_task_status_section_label: "タスクステータスを設定",
        to_do_button_tooltip: "やること",
        in_progress_button_tooltip: "進行中",
        done_button_tooltip: "完了",
        task_actions_section_label: "タスクの操作",
        duplicate_task_button_tooltip: "タスクを複製",
        archive_task_button_tooltip: "タスクをアーカイブ",
        unarchive_task_button_tooltip: "タスクを復元",
        assignees_section_label: "担当者",
        assign_user_toggle_button_tooltip: "ユーザーを割り当て",
        toggle_user_filter_button_label: "ユーザーフィルターを切り替え",
        assignee_selection_section_label: "担当者の選択",
        add_user_button_label: "ユーザーを追加",
        add_user_form_label: "ユーザーを追加",
        user_name_input_label: "名前",
        cancel_adding_new_user_button_label: "新しいユーザーの追加をキャンセル",
        remove_user_from_task_button_label: "タスクからユーザーを削除",
        tags_section_label: "タグ",
        tag_selection_section_label: "タグの選択",
        add_tag_button_label: "タグを追加",
        add_tag_form_label: "タグを追加",
        tag_name_input_label: "名前",
        add_tag_toggle_button_tooltip: "タグを追加",
        cancel_adding_new_tag_button_label: "タグの追加をキャンセル",
        toggle_tag_filter_button_label: "タグフィルターを切り替え",
        remove_tag_from_task_button_label: "タスクからタグを削除",
        toggle_expand_task_button_label: "タスクを拡張",
        due_date_section_label: "期日",
        edit_due_date_tooltip: "期日を編集",
        due_date_form_label: "期日を設定",
        due_date_input_label: "期日",
        set_due_date_button_label: "期日を設定",
        cancel_due_date_update_button_label: "期日更新をキャンセル",
        color_picker_legend_label: "色",
        description_update_form_label: "説明を更新",
        set_description_button_label: "説明を設定",
        cancel_description_update_button_label: "説明の更新をキャンセル",
        bullet_points_button_tooltip: "箇条書き",
        task_list_button_tooltip: "タスクリスト",
        description_text_area_label: "説明",
        description_section_label: "説明",
        edit_description_tooltip: "説明を編集",
        additional_actions_section_label: "追加の操作",
        delete_task_tooltip: "タスクを削除",
        edit_tag_color_form_label: "色を編集",
        edit_tag_color_button_label: "色を編集",
        set_tag_color_button_label: "色を設定",
        cancel_tag_color_update_label: "色の更新をキャンセル",
        edit_tag_name_button_label: "名前を編集",
        edit_tag_name_form_label: "名前を編集",
        set_tag_name_button_label: "名前を設定",
        cancel_tag_name_update_button_label: "名前の更新をキャンセル",
        delete_tag_button_label: "タグを削除",
        archive_tag_button_label: "タグをアーカイブ",
        unarchive_tag_button_label: "タグを復元",
        edit_user_color_form_label: "色を編集",
        set_user_color_button_label: "色を設定",
        cancel_user_color_update_button_label: "色の更新をキャンセル",
        edit_user_color_button_label: "色を編集",
        edit_user_name_form_label: "名前を編集",
        set_user_name_button_label: "名前を設定",
        cancel_user_name_update_button_label: "名前の更新をキャンセル",
        edit_user_name_button_label: "名前を編集",
        delete_user_button_label: "ユーザーを削除",
        task_status_section_label: "タスクのステータス",
        filters_section_label: "フィルター",
        languages_section_title: "言語",
        custom_task_button_label: "カスタムタスク",
        board_list_section_label: "ボードリスト",
        join_board_button_label: "ボードに参加",
        create_new_board_button_label: "新しいボードを作成",
        or_label: "または",
        chat_gpt_limit_exceeded_title: "ChatGPTの制限を超えました",
        chat_gpt_limit_exceeded_content: "ChatGPTの呼び出し制限に達しました。後でもう一度お試しください。",
        chat_gpt_waiting_message: "ChatGPTとの対話中...",
        chat_gpt_error_title: "ChatGPTエラー",
        chat_gpt_error_content: "ChatGPTとの接続中にエラーが発生しました。後でもう一度お試しください。",
        chat_gpt_prompt_input_title: "ChatGPTのプロンプト",
        chat_gpt_daily_attempts_left: "残りの毎日の試行回数",
        chat_gpt_prompt_input_content: "以下の提案の中から選択するか:",
        chat_gpt_prompt_input_form_label: "chat gptプロンプト",
        chat_gpt_prompt_input_label: "プロンプト:",
        suggest_cupcake_recipe_prompt: "カップケーキのレシピを提案",
        paint_bedroom_prompt: "寝室をペイント",
        friends_over_for_bbq_prompt: "バーベキューに友達を招く",
        prepare_for_rome_vacation_prompt: "ローマ旅行の準備",
        house_tidy_prompt: "家の整理",
        fix_fence_prompt: "フェンスの修理",
        join_board_form_label: "ボードに参加",
        join_board_input_label: "ボード名",
        cancel_joining_board_button_label: "ボード参加をキャンセル",
        add_task_button_label: "タスクを追加",
        remove_board_button_label: "ボードを削除",
        new_task_form_label: "新しいタスク",
        cancel_adding_new_task_button_label: "新規タスクの追加をキャンセル",
        navigation_section_label: "ナビゲーション",
        toggle_actions_drawer_button_label: "アクション引き出しを切り替え",
        toggle_show_filters_button_label: "フィルター表示を切り替え",
        themes_section_label: "テーマ",
        close_theme_selector_button_label: "テーマ選択を閉じる",
        close_filters_button_label: "フィルターを閉じる",
        board_link: "ボード",
        tags_link: "タグ",
        users_link: "ユーザー",
        archive_link: "アーカイブ",
    },
};
