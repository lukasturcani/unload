CREATE TABLE
IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created INTEGER NOT NULL,
    updated INTEGER NOT NULL,
    due INTEGER,
    size TEXT CHECK (size IN ('SMALL', 'MEDIUM', 'LARGE')) NOT NULL,
    status TEXT CHECK (status IN ('TODO', 'IN_PROGRESS', 'DONE')) NOT NULL
);
