CREATE INDEX
IF NOT EXISTS task_assignments_task_id_user_id_idx ON task_assignments (
    board_name,
    task_id,
    user_id
);
