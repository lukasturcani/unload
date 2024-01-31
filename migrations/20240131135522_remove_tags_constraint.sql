PRAGMA defer_foreign_keys = ON;
CREATE TABLE IF NOT EXISTS new_tags (
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
  FOREIGN KEY (board_name) REFERENCES boards (name)
);
INSERT INTO new_tags SELECT * FROM tags;
DROP TABLE tags;
ALTER TABLE new_tags RENAME TO tags;
ALTER TABLE tags
ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE;
CREATE INDEX IF NOT EXISTS tags_tag_idx ON tags (
    board_name,
    id
);
CREATE INDEX IF NOT EXISTS tags_board_name_archived_idx ON tags (
  board_name,
  archived
);
PRAGMA foreign_key_check;
PRAGMA defer_foreign_keys = OFF;
