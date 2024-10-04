use super::{Text, Translation};

pub const PT: Translation = Translation {
    id: "pt",
    name: "PT - Português",
    text: Text {
        to_do_column_title: "A Fazer",
        in_progress_column_title: "Em Progresso",
        done_column_title: "Concluído",
        pick_language_tooltip: "Selecionar Idioma",
        toggle_show_themes_tooltip: "Alterar Tema",
        toggle_dense_view_tooltip: "Alternar Visualização Compacta",
        edit_board_title_tooltip: "Editar Título",
        board_title_input_label: "Título",
        board_title_update_form_label: "Atualizar Título do Quadro",
        set_board_title_button_label: "Definir Título",
        cancel_board_title_update_button_label: "Cancelar Atualização do Título",
        task_title_input_label: "Título",
        edit_task_title_tooltip: "Editar Título",
        task_title_update_form_label: "Atualizar Título da Tarefa",
        set_task_title_button_label: "Definir Título",
        cancel_task_title_update_button_label: "Cancelar Atualização do Título",
        set_task_status_section_label: "Definir Status da Tarefa",
        to_do_button_tooltip: "A Fazer",
        in_progress_button_tooltip: "Em Progresso",
        done_button_tooltip: "Concluído",
        task_actions_section_label: "Ações da Tarefa",
        duplicate_task_button_tooltip: "Duplicar Tarefa",
        archive_task_button_tooltip: "Arquivar Tarefa",
        unarchive_task_button_tooltip: "Restaurar Tarefa",
        assignees_section_label: "Responsáveis",
        assign_user_toggle_button_tooltip: "Atribuir Usuário",
        toggle_user_filter_button_label: "Alternar Filtro de Usuário",
        assignee_selection_section_label: "Seleção de Responsáveis",
        add_user_button_label: "Adicionar Usuário",
        add_user_form_label: "Adicionar Usuário",
        cancel_adding_new_user_button_label: "Cancelar Adição de Usuário",
        user_name_input_label: "Nome",
        remove_user_from_task_button_label: "Remover Usuário da Tarefa",
        tags_section_label: "Etiquetas",
        tag_selection_section_label: "Seleção de Etiquetas",
        add_tag_button_label: "Adicionar Etiqueta",
        add_tag_form_label: "Adicionar Etiqueta",
        tag_name_input_label: "Nome",
        add_tag_toggle_button_tooltip: "Adicionar Etiqueta",
        cancel_adding_new_tag_button_label: "Cancelar Adição de Etiqueta",
        toggle_tag_filter_button_label: "Alternar Filtro de Etiqueta",
        remove_tag_from_task_button_label: "Remover Etiqueta da Tarefa",
        toggle_expand_task_button_label: "Alternar Expansão da Tarefa",
        due_date_section_label: "Data de Vencimento",
        edit_due_date_tooltip: "Editar Data de Vencimento",
        due_date_form_label: "Definir Data de Vencimento",
        due_date_input_label: "Data",
        set_due_date_button_label: "Definir Data",
        cancel_due_date_update_button_label: "Cancelar Atualização da Data",
        color_picker_legend_label: "Cor",
        description_update_form_label: "Atualizar Descrição",
        set_description_button_label: "Definir Descrição",
        cancel_description_update_button_label: "Cancelar Atualização da Descrição",
        bullet_points_button_tooltip: "Marcadores",
        task_list_button_tooltip: "Lista de Tarefas",
        description_text_area_label: "Descrição",
        description_section_label: "Descrição",
        edit_description_tooltip: "Editar Descrição",
        additional_actions_section_label: "Ações Adicionais",
        delete_task_tooltip: "Excluir Tarefa",
        edit_tag_color_form_label: "Editar Cor",
        edit_tag_color_button_label: "Editar Cor",
        set_tag_color_button_label: "Definir Cor",
        cancel_tag_color_update_label: "Cancelar Atualização da Cor",
        edit_tag_name_button_label: "Editar Nome",
        edit_tag_name_form_label: "Editar Nome",
        set_tag_name_button_label: "Definir Nome",
        cancel_tag_name_update_button_label: "Cancelar Atualização do Nome",
        delete_tag_button_label: "Excluir Etiqueta",
        archive_tag_button_label: "Arquivar Etiqueta",
        unarchive_tag_button_label: "Desarquivar Etiqueta",
        edit_user_color_form_label: "Editar Cor do Usuário",
        set_user_color_button_label: "Definir Cor do Usuário",
        cancel_user_color_update_button_label: "Cancelar Atualização da Cor",
        edit_user_color_button_label: "Editar Cor do Usuário",
        edit_user_name_form_label: "Editar Nome",
        set_user_name_button_label: "Definir Nome",
        cancel_user_name_update_button_label: "Cancelar Atualização do Nome",
        edit_user_name_button_label: "Editar Nome",
        delete_user_button_label: "Excluir Usuário",
        task_status_section_label: "Status da Tarefa",
        filters_section_label: "Filtros",
        languages_section_title: "Idiomas",
        board_list_section_label: "Lista de Quadros",
        custom_task_button_label: "Tarefa Personalizada",
        join_board_button_label: "Entrar no Quadro",
        create_new_board_button_label: "Criar Novo Quadro",
        chat_gpt_limit_exceeded_title: "Limite do ChatGPT Excedido",
        or_label: "ou",
        chat_gpt_limit_exceeded_content:
            "Você atingiu o limite de chamadas ao ChatGPT. Tente novamente mais tarde.",
        chat_gpt_waiting_message: "Conversando com o ChatGPT...",
        chat_gpt_error_title: "Erro do ChatGPT",
        chat_gpt_error_content:
            "Ocorreu um erro ao tentar conectar ao ChatGPT. Tente novamente mais tarde.",
        chat_gpt_prompt_input_title: "Prompt do ChatGPT",
        chat_gpt_daily_attempts_left: "tentativas diárias restantes",
        chat_gpt_prompt_input_content: "ou escolha uma das sugestões abaixo:",
        chat_gpt_prompt_input_form_label: "prompt do chat gpt",
        suggest_cupcake_recipe_prompt: "sugerir receita de cupcake",
        paint_bedroom_prompt: "pintar o quarto",
        friends_over_for_bbq_prompt: "amigos para um churrasco",
        prepare_for_rome_vacation_prompt: "preparar para férias em Roma",
        house_tidy_prompt: "organizar a casa",
        fix_fence_prompt: "consertar cerca",
        chat_gpt_prompt_input_label: "Prompt:",
        join_board_form_label: "Entrar no Quadro",
        join_board_input_label: "Nome do Quadro",
        cancel_joining_board_button_label: "Cancelar Entrada no Quadro",
        add_task_button_label: "Adicionar Tarefa",
        remove_board_button_label: "Remover Quadro",
        new_task_form_label: "Nova Tarefa",
        cancel_adding_new_task_button_label: "Cancelar Adição de Nova Tarefa",
        navigation_section_label: "Navegação",
        toggle_actions_drawer_button_label: "Alternar Gaveta de Ações",
        toggle_show_filters_button_label: "Alternar Exibição de Filtros",
        themes_section_label: "Temas",
        close_theme_selector_button_label: "Fechar Seletor de Tema",
        close_filters_button_label: "Fechar Filtros",
        board_link: "Quadro",
        tags_link: "Etiquetas",
        users_link: "Usuários",
        archive_link: "Arquivo",
    },
};