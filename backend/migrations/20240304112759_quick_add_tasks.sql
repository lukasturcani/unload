CREATE TABLE IF NOT EXISTS quick_add_tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    size TEXT CHECK (size IN ('Small', 'Medium', 'Large')) NOT NULL,
    FOREIGN KEY (board_name) REFERENCES boards (name)
);
CREATE INDEX
IF NOT EXISTS quick_add_tasks_board_name_idx
ON quick_add_tasks (board_name);

CREATE TABLE IF NOT EXISTS quick_add_task_assignments (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    UNIQUE (user_id, task_id),
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (task_id) REFERENCES quick_add_tasks (id)
);

CREATE INDEX
IF NOT EXISTS quick_add_task_assignments_board_name_idx
ON quick_add_task_assignments (board_name);

CREATE INDEX
IF NOT EXISTS quick_add_task_assignments_task_id_idx
ON quick_add_task_assignments (board_name, task_id);

CREATE INDEX
IF NOT EXISTS quick_add_task_assignments_user_id_idx
ON quick_add_task_assignments (board_name, user_id);


CREATE TABLE IF NOT EXISTS quick_add_task_tags (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    task_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    UNIQUE (task_id, tag_id),
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (task_id) REFERENCES quick_add_tasks (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);

CREATE INDEX
IF NOT EXISTS quick_add_task_tags_board_name_idx
ON quick_add_task_tags (board_name);

CREATE INDEX
IF NOT EXISTS quick_add_task_tags_task_id_idx
ON quick_add_task_tags (board_name, task_id);

CREATE INDEX
IF NOT EXISTS quick_add_task_tags_tag_id_idx
ON quick_add_task_tags (board_name, tag_id);
