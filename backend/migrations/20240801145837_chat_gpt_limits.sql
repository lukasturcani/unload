-- Add migration script here
CREATE TABLE IF NOT EXISTS chat_gpt_limits (
    id INTEGER PRIMARY KEY NOT NULL,
    board_name TEXT NOT NULL,
    calls_left INTEGER NOT NULL,
    UNIQUE (board_name),
    FOREIGN KEY (board_name) REFERENCES boards (name)
);

CREATE INDEX IF NOT EXISTS chat_gpt_limits_board_name_idx ON chat_gpt_limits (
  board_name
);
