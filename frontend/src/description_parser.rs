#[derive(Clone, Eq, PartialEq)]
pub struct Line {
    pub index: usize,
    pub content: String,
}

pub enum Block {
    Text(String),
    Bullet(Vec<String>),
    Checkbox(Vec<Line>),
}

pub fn parse_blocks(description: &str) -> Vec<Block> {
    let mut blocks = Vec::<Block>::new();
    let mut line_index = 0;
    for line in description.lines() {
        if line.starts_with("* ") {
            if let Some(Block::Bullet(lines)) = blocks.last_mut() {
                lines.push(line.into());
            } else {
                blocks.push(Block::Bullet(vec![line.into()]));
            };
        } else if line.starts_with("- [ ]") || line.starts_with("- [x]") {
            if let Some(Block::Checkbox(lines)) = blocks.last_mut() {
                lines.push(Line {
                    index: line_index,
                    content: line.into(),
                });
            } else {
                blocks.push(Block::Checkbox(vec![Line {
                    index: line_index,
                    content: line.into(),
                }]));
            };
        } else if let Some(Block::Text(text)) = blocks.last_mut() {
            text.push('\n');
            text.push_str(line);
        } else {
            blocks.push(Block::Text(line.into()));
        };
        line_index += line.len() + 1;
    }
    blocks
}
