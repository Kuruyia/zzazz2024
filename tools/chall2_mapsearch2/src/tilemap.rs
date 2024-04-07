use std::collections::HashMap;
use crate::error::Error;

#[derive(Debug, Default)]
pub struct Tilemap {
    tilemap: HashMap<(usize, usize), u8>,
}

impl Tilemap {
    pub fn tilemap(&self) -> &HashMap<(usize, usize), u8> {
        &self.tilemap
    }

    pub fn build_tilemap(&mut self, width: usize/*, height: usize*/, blocks: &Vec<u8>, block_definitions: &HashMap<u8, Vec<u8>>) -> Result<(), Error> {
        let block_width = 4usize;
        let block_height = 4usize;
        // let tile_row_tile_count = width * block_width;
        // let block_row_tile_count = tile_row_tile_count * block_height;

        // self.tilemap.resize(width * block_width * height * block_height, 0);

        for (block_offset, block) in blocks.iter().enumerate() {
            let tiles = block_definitions.get(block).ok_or(Error::BlockNotFound)?;
            let block_x = block_offset % width;
            let block_y = block_offset / width;
            let tile_width_offset = block_x * block_width;
            let tile_height_offset = block_y * block_height;

            for (tile_offset, tile) in tiles.iter().enumerate() {
                let tile_x = tile_offset % block_width;
                let tile_y = tile_offset / block_width;
                // let tile_nb = (tile_y * tile_row_tile_count + block_y * block_row_tile_count) + (tile_x + tile_width_offset);

                // self.tilemap[tile_nb] = *tile;
                self.tilemap.insert((tile_x + tile_width_offset, tile_y + tile_height_offset), *tile);
            }
        }

        Ok(())
    }

    pub fn set_tile_at(&mut self, x: usize, y: usize, tile: u8) {
        self.tilemap.insert((x, y), tile);
    }

    pub fn set_tilemap(&mut self, tilemap: HashMap<(usize, usize), u8>) {
        self.tilemap = tilemap;
    }

    pub fn width(&self) -> usize {
        let max_item = self.tilemap.iter()
            .max_by(|(coords_lhs, _), (coords_rhs, _)| coords_lhs.0.cmp(&coords_rhs.0))
            .unwrap();

        max_item.0.0 + 1
    }

    pub fn height(&self) -> usize {
        let max_item = self.tilemap.iter()
            .max_by(|(coords_lhs, _), (coords_rhs, _)| coords_lhs.1.cmp(&coords_rhs.1))
            .unwrap();

        max_item.0.1 + 1
    }
}