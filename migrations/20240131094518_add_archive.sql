ALTER TABLE tasks
ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE;

DROP INDEX IF EXISTS tasks_board_name_idx;
CREATE INDEX IF NOT EXISTS tasks_board_name_archived_idx ON tasks (board_name, archived);
