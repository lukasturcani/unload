DROP TABLE IF EXISTS task_dependencies;

CREATE TABLE
IF NOT EXISTS repeat_daily (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    task_id INTEGER NOT NULL,
    every_n_days INTEGER NOT NULL,
    from_date DATE NOT NULL,
    last_done DATE,
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (task_id) REFERENCES tasks (id)
);

CREATE TABLE
IF NOT EXISTS repeat_weekly (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    task_id INTEGER NOT NULL,
    day_of_week TEXT CHECK (day_of_week IN (
        'Sunday',
        'Monday',
        'Tuesday',
        'Wednesday',
        'Thursday',
        'Friday',
        'Saturday'
    )) NOT NULL,
    every_n_weeks INTEGER NOT NULL,
    from_date DATE NOT NULL,
    last_done DATE,
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (task_id) REFERENCES tasks (id)
);

CREATE TABLE
IF NOT EXISTS repeat_monthly (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    task_id INTEGER NOT NULL,
    day_of_month INTEGER NOT NULL,
    every_n_months INTEGER NOT NULL,
    from_date DATE NOT NULL,
    last_done DATE,
    FOREIGN KEY (board_name) REFERENCES boards (name),
    FOREIGN KEY (task_id) REFERENCES tasks (id)
);
