DROP INDEX IF EXISTS task_assignments_user_id_idx;
DROP INDEX IF EXISTS task_assignments_task_id_idx;
DROP INDEX IF EXISTS task_dependencies_task_id_idx;
DROP INDEX IF EXISTS task_dependencies_blocks_id_idx;

CREATE INDEX IF NOT EXISTS tasks_task_idx ON tasks (
    board_name, id
);

CREATE INDEX IF NOT EXISTS users_user_idx ON users (
    board_name, id
);

CREATE INDEX
IF NOT EXISTS task_assignments_task_id_idx ON task_assignments (
    board_name, task_id
);
CREATE INDEX
IF NOT EXISTS task_assignments_user_id_idx ON task_assignments (
    board_name, user_id
);

CREATE INDEX
IF NOT EXISTS task_dependencies_task_id_idx ON task_dependencies (
    board_name, task_id
);
CREATE INDEX
IF NOT EXISTS task_dependencies_blocks_id_idx ON task_dependencies (
    board_name, blocks_id
);
