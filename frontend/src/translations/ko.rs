use super::{Text, Translation};

pub const KO: Translation = Translation {
    id: "ko",
    name: "KO - 한국어",
    text: Text {
        to_do_column_title: "할 일",
        in_progress_column_title: "진행 중",
        done_column_title: "완료됨",
        pick_language_tooltip: "언어 선택",
        toggle_show_themes_tooltip: "테마 변경",
        toggle_dense_view_tooltip: "밀집 보기 전환",
        edit_board_title_tooltip: "제목 편집",
        board_title_input_label: "제목",
        board_title_update_form_label: "보드 제목 업데이트",
        set_board_title_button_label: "제목 설정",
        cancel_board_title_update_button_label: "제목 업데이트 취소",
        task_title_input_label: "제목",
        edit_task_title_tooltip: "제목 편집",
        task_title_update_form_label: "작업 제목 업데이트",
        set_task_title_button_label: "제목 설정",
        cancel_task_title_update_button_label: "작업 제목 업데이트 취소",
        set_task_status_section_label: "작업 상태 설정",
        to_do_button_tooltip: "할 일",
        in_progress_button_tooltip: "진행 중",
        done_button_tooltip: "완료됨",
        task_actions_section_label: "작업 액션",
        duplicate_task_button_tooltip: "작업 복제",
        archive_task_button_tooltip: "작업 보관",
        unarchive_task_button_tooltip: "작업 복원",
        assignees_section_label: "담당자",
        assign_user_toggle_button_tooltip: "사용자 할당",
        toggle_user_filter_button_label: "사용자 필터 전환",
        assignee_selection_section_label: "담당자 선택",
        add_user_button_label: "사용자 추가",
        add_user_form_label: "사용자 추가",
        cancel_adding_new_user_button_label: "사용자 추가 취소",
        user_name_input_label: "이름",
        remove_user_from_task_button_label: "작업에서 사용자 제거",
        tags_section_label: "태그",
        tag_selection_section_label: "태그 선택",
        add_tag_button_label: "태그 추가",
        add_tag_form_label: "태그 추가",
        tag_name_input_label: "이름",
        add_tag_toggle_button_tooltip: "태그 추가",
        cancel_adding_new_tag_button_label: "태그 추가 취소",
        toggle_tag_filter_button_label: "태그 필터 전환",
        remove_tag_from_task_button_label: "작업에서 태그 제거",
        toggle_expand_task_button_label: "작업 확장 전환",
        due_date_section_label: "마감일",
        edit_due_date_tooltip: "마감일 수정",
        due_date_form_label: "마감일 설정",
        due_date_input_label: "마감일",
        set_due_date_button_label: "마감일 설정",
        cancel_due_date_update_button_label: "마감일 업데이트 취소",
        color_picker_legend_label: "색상",
        description_update_form_label: "설명 업데이트",
        set_description_button_label: "설명 설정",
        cancel_description_update_button_label: "설명 업데이트 취소",
        bullet_points_button_tooltip: "글머리 기호",
        task_list_button_tooltip: "작업 목록",
        description_text_area_label: "설명",
        description_section_label: "설명",
        edit_description_tooltip: "설명 편집",
        additional_actions_section_label: "추가 작업",
        delete_task_tooltip: "작업 삭제",
        edit_tag_color_form_label: "색상 편집",
        edit_tag_color_button_label: "색상 편집",
        set_tag_color_button_label: "색상 설정",
        cancel_tag_color_update_label: "색상 업데이트 취소",
        edit_tag_name_button_label: "이름 편집",
        edit_tag_name_form_label: "이름 편집",
        set_tag_name_button_label: "이름 설정",
        cancel_tag_name_update_button_label: "이름 업데이트 취소",
        delete_tag_button_label: "태그 삭제",
        archive_tag_button_label: "태그 보관",
        unarchive_tag_button_label: "태그 복원",
        edit_user_color_form_label: "사용자 색상 편집",
        set_user_color_button_label: "사용자 색상 설정",
        cancel_user_color_update_button_label: "사용자 색상 업데이트 취소",
        edit_user_color_button_label: "사용자 색상 편집",
        edit_user_name_form_label: "사용자 이름 편집",
        set_user_name_button_label: "사용자 이름 설정",
        cancel_user_name_update_button_label: "사용자 이름 업데이트 취소",
        edit_user_name_button_label: "사용자 이름 편집",
        delete_user_button_label: "사용자 삭제",
        task_status_section_label: "작업 상태",
        filters_section_label: "필터",
        languages_section_title: "언어",
        board_list_section_label: "보드 목록",
        custom_task_button_label: "사용자 정의 작업",
        join_board_button_label: "보드 참가",
        create_new_board_button_label: "새 보드 생성",
        chat_gpt_limit_exceeded_title: "ChatGPT 한도 초과",
        or_label: "또는",
        chat_gpt_limit_exceeded_content:
            "ChatGPT 호출 한도에 도달했습니다. 나중에 다시 시도해 주세요.",
        chat_gpt_waiting_message: "ChatGPT와 대화 중...",
        chat_gpt_error_title: "ChatGPT 오류",
        chat_gpt_error_content:
            "ChatGPT에 연결하는 동안 오류가 발생했습니다. 나중에 다시 시도해 주세요.",
        chat_gpt_prompt_input_title: "ChatGPT 프롬프트",
        chat_gpt_daily_attempts_left: "남은 일일 시도 횟수",
        chat_gpt_prompt_input_content: "또는 아래 제안에서 선택하세요:",
        chat_gpt_prompt_input_form_label: "chat gpt 프롬프트",
        suggest_cupcake_recipe_prompt: "컵케이크 레시피 추천",
        paint_bedroom_prompt: "침실 페인팅",
        friends_over_for_bbq_prompt: "BBQ 친구 초대",
        prepare_for_rome_vacation_prompt: "로마 휴가 준비",
        house_tidy_prompt: "집 정리",
        fix_fence_prompt: "울타리 수리",
        chat_gpt_prompt_input_label: "프롬프트:",
        join_board_form_label: "보드 참가",
        join_board_input_label: "보드 이름",
        cancel_joining_board_button_label: "보드 참가 취소",
        add_task_button_label: "작업 추가",
        remove_board_button_label: "보드 삭제",
        new_task_form_label: "새 작업",
        cancel_adding_new_task_button_label: "새 작업 추가 취소",
        navigation_section_label: "탐색",
        toggle_actions_drawer_button_label: "작업 서랍 전환",
        toggle_show_filters_button_label: "필터 보기 전환",
        themes_section_label: "테마",
        close_theme_selector_button_label: "테마 선택기 닫기",
        close_filters_button_label: "필터 닫기",
        board_link: "보드",
        tags_link: "태그",
        users_link: "사용자",
        archive_link: "보관함",
    },
};