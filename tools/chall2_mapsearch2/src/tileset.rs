use std::collections::HashMap;

#[derive(Debug)]
pub struct Tileset {
    filename: String,
    blocks: HashMap<u8, Vec<u8>>,
}

impl Tileset {
    pub fn new(filename: String, blocks: HashMap<u8, Vec<u8>>) -> Self {
        Self {
            filename,
            blocks,
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn blocks(&self) -> &HashMap<u8, Vec<u8>> {
        &self.blocks
    }
}