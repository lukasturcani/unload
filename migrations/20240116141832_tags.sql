CREATE TABLE tags (
    name TEXT PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    FOREIGN KEY (board_name) REFERENCES boards (name),
    UNIQUE (name, board_name)
);
CREATE INDEX IF NOT EXISTS tags_board_name_idx ON tags (
    board_name
);

CREATE TABLE IF NOT EXISTS task_tags (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    task_id INTEGER NOT NULL,
    tag_name TEXT NOT NULL,
    UNIQUE (task_id, tag_name),
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (task_id) REFERENCES tasks (id),
    FOREIGN KEY (tag_name) REFERENCES tags (name)
);
CREATE INDEX IF NOT EXISTS task_tags_board_name_idx ON task_tags (
    board_name
);
