INSERT INTO chat_gpt_limits (board_name, calls_left)
SELECT boards.name, 20
FROM boards
LEFT JOIN chat_gpt_limits ON boards.name = chat_gpt_limits.board_name
WHERE chat_gpt_limits.board_name IS NULL;
