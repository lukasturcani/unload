CREATE TABLE
IF NOT EXISTS boards (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    title TEXT NOT NULL
);


CREATE TABLE
IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    board_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created INTEGER NOT NULL,
    updated INTEGER NOT NULL,
    due INTEGER,
    size TEXT CHECK (size IN ('SMALL', 'MEDIUM', 'LARGE')) NOT NULL,
    status TEXT CHECK (status IN ('TO_DO', 'IN_PROGRESS', 'DONE')) NOT NULL,
    FOREIGN KEY (board_id) REFERENCES boards (id)
);
CREATE INDEX IF NOT EXISTS tasks_board_id_idx ON tasks (board_id);


CREATE TABLE
IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    board_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    color TEXT CHECK (color IN (

        'BLACK',
        'WHITE',
        'GRAY',
        'SILVER',
        'MAROON',
        'RED',
        'PURPLE',
        'FUSHSIA',
        'GREEN',
        'LIME',
        'OLIVE',
        'YELLOW',
        'NAVY',
        'BLUE',
        'TEAL',
        'AQUA'

    )) NOT NULL,
    FOREIGN KEY (board_id) REFERENCES boards (id)
);
CREATE INDEX IF NOT EXISTS users_board_id_idx ON users (board_id);


CREATE TABLE
IF NOT EXISTS task_assignments (
    id INTEGER PRIMARY KEY NOT NULL,
    board_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    UNIQUE (user_id, task_id),
    FOREIGN KEY (board_id) REFERENCES boards (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (task_id) REFERENCES tasks (id)
);
CREATE INDEX IF NOT EXISTS task_assignments_board_id_idx ON task_assignments (
    board_id
);
CREATE INDEX IF NOT EXISTS task_assignments_user_id_idx ON task_assignments (
    user_id
);
CREATE INDEX IF NOT EXISTS task_assignments_task_id_idx ON task_assignments (
    task_id
);


CREATE TABLE IF NOT EXISTS task_dependencies (
    id INTEGER PRIMARY KEY NOT NULL,
    board_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    blocks_id INTEGER NOT NULL,
    UNIQUE (task_id, blocks_id),
    FOREIGN KEY (board_id) REFERENCES boards (id),
    FOREIGN KEY (task_id) REFERENCES tasks (id),
    FOREIGN KEY (blocks_id) REFERENCES tasks (id)
);
CREATE INDEX IF NOT EXISTS task_dependencies_board_id_idx ON task_dependencies (
    board_id
);
CREATE INDEX IF NOT EXISTS task_dependencies_task_id_idx ON task_dependencies (
    task_id
);
CREATE INDEX
IF NOT EXISTS task_dependencies_blocks_id_idx ON task_dependencies (
    blocks_id
);
