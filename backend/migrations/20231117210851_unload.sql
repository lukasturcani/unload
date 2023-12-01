CREATE TABLE nouns (
    id INTEGER PRIMARY KEY NOT NULL,
    noun TEXT NOT NULL,
    UNIQUE (noun)

);

CREATE TABLE adjectives (
    id INTEGER PRIMARY KEY NOT NULL,
    adjective TEXT NOT NULL,
    UNIQUE (adjective)

);

CREATE TABLE
IF NOT EXISTS boards (
    name TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL
);


CREATE TABLE
IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created DATETIME NOT NULL,
    updated DATETIME NOT NULL,
    due DATETIME,
    size TEXT CHECK (size IN ('Small', 'Medium', 'Large')) NOT NULL,
    status TEXT CHECK (status IN ('ToDo', 'InProgress', 'Done')) NOT NULL,
    FOREIGN KEY (board_name) REFERENCES boards (name)
);
CREATE INDEX IF NOT EXISTS tasks_board_name_idx ON tasks (board_name);


CREATE TABLE
IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    name TEXT NOT NULL,
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
    UNIQUE (board_name, name, color),
    FOREIGN KEY (board_name) REFERENCES boards (name)
);
CREATE INDEX IF NOT EXISTS users_board_name_idx ON users (board_name);


CREATE TABLE
IF NOT EXISTS task_assignments (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    UNIQUE (user_id, task_id),
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (task_id) REFERENCES tasks (id)
);
CREATE INDEX IF NOT EXISTS task_assignments_board_name_idx ON task_assignments (
    board_name
);
CREATE INDEX IF NOT EXISTS task_assignments_user_id_idx ON task_assignments (
    user_id
);
CREATE INDEX IF NOT EXISTS task_assignments_task_id_idx ON task_assignments (
    task_id
);


CREATE TABLE IF NOT EXISTS task_dependencies (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    task_id INTEGER NOT NULL,
    blocks_id INTEGER NOT NULL,
    UNIQUE (task_id, blocks_id),
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (task_id) REFERENCES tasks (id),
    FOREIGN KEY (blocks_id) REFERENCES tasks (id)
);
CREATE INDEX IF NOT EXISTS task_dependencies_board_name_idx
ON task_dependencies (
    board_name
);
CREATE INDEX IF NOT EXISTS task_dependencies_task_id_idx ON task_dependencies (
    task_id
);
CREATE INDEX
IF NOT EXISTS task_dependencies_blocks_id_idx ON task_dependencies (
    blocks_id
);
