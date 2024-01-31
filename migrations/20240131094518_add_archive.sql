ALTER TABLE tasks
ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE;

DROP INDEX IF EXISTS tasks_board_name_idx;
CREATE INDEX IF NOT EXISTS tasks_board_name_archived_idx ON tasks (board_name, archived);

ALTER TABLE tags
ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE;

DROP INDEX IF EXISTS tags_board_name_idx;
CREATE INDEX IF NOT EXISTS tags_board_name_archived_idx ON tags (board_name, archived);
