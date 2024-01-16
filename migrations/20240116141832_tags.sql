CREATE TABLE
IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    board_name TEXT NOT NULL,
    color TEXT CHECK (color IN (
        'Black',
        'White',
        'Gray',
        'Silver',
        'Maroon',
        'Red',
        'Purple',
        'Fushsia',
        'Green',
        'Lime',
        'Olive',
        'Yellow',
        'Navy',
        'Blue',
        'Teal',
        'Aqua'

    )) NOT NULL,
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
    tag_id INTEGER NOT NULL,
    UNIQUE (task_id, tag_id),
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (task_id) REFERENCES tasks (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);
CREATE INDEX IF NOT EXISTS task_tags_board_name_idx ON task_tags (
    board_name
);
CREATE INDEX IF NOT EXISTS task_tags_task_id_idx ON task_tags (
    board_name,
    task_id
);
