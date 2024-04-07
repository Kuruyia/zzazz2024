use std::{fs, io};
use std::collections::HashMap;

use regex::Regex;

use crate::error::Error;
use crate::map::Map;
use crate::tileset::Tileset;

mod tileset;
mod map;
mod error;
mod tilemap;

fn load_tilesets() -> Result<Vec<Tileset>, Error> {
    // Get all the block file paths
    let block_file_paths = fs::read_dir("../../../pokecrystal-master/data/tilesets")
        .map_err(Error::IoError)?
        .map(|res| res.map(|e| e.path()))
        .filter(|v| v.as_ref().unwrap().extension().unwrap().eq("bin"))
        .collect::<Result<Vec<_>, io::Error>>()
        .map_err(Error::IoError)?;

    let mut res = Vec::<Tileset>::new();

    for path in block_file_paths {
        let file_contents = fs::read(path.clone()).map_err(Error::IoError)?;
        let mut blocks: HashMap<u8, Vec<u8>> = HashMap::new();

        for (i, tile) in file_contents.iter().enumerate() {
            let block_pos = i / 16;
            let values = blocks.entry(block_pos.try_into().unwrap()).or_default();

            values.push(*tile);
        }

        let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
        res.push(Tileset::new(filename.clone(), blocks));
    }

    Ok(res)
}

fn load_maps() -> Result<Vec<Map>, Error> {
    let mut res = Vec::<Map>::new();
    let map_spec = fs::read_to_string("../../../pokecrystal-master/data/maps/maps.asm").map_err(Error::IoError)?;
    let re = Regex::new(r"(?m)map\s*(.*?),\s*(.*?),", ).map_err(Error::RegexError)?;

    for line in map_spec.lines() {
        let captures = re.captures(line);

        if let Some(captures) = captures {
            res.push(Map::new(captures[1].to_owned(), captures[2].to_owned()));
        }
    }

    Ok(res)
}

fn check_map_constraints(map: &Map) -> Result<Vec<(isize, isize)>, Error> {
    let map_width = *map.width() as isize;
    let map_height = *map.height() as isize;
    let mut res = Vec::<(isize, isize)>::new();

    for x in 0..map_width * 2 {
        for y in 0..map_height * 2 {
            let tiles = map.get_tilemap_at_player_pos(x, y)?;

            if tiles[0x0B] == 0x05 && tiles[0x7F] == 0x23 && tiles[0x2B] == 0x02 && tiles[0x2C] == 0x04 && tiles[0xE8] == 0x01 {
                res.push((x, y));
            }
        }
    }

    Ok(res)
}

fn main() -> Result<(), Error> {
    // Get the tilesets and maps
    println!("Loading tilesets and maps...");
    let tilesets = load_tilesets()?;
    let mut maps = load_maps()?;
    println!("Loaded {} maps and {} tilesets", maps.len(), tilesets.len());

    // TODO: Remove me
    // maps.retain(|v| v.name() == "Route39");

    // Load the blocks and size of each map
    println!("Loading map extended data...");
    for map in &mut maps {
        map.load_blocks(&"../../../pokecrystal-master/maps/")?;
        map.load_attributes(&"../../../pokecrystal-master/data/maps/attributes.asm")?;
        map.load_size(&"../../../pokecrystal-master/constants/map_constants.asm")?;
    }

    // Build the tilemap of each map
    println!("Building map tilemaps...");
    for map in &mut maps {
        let tileset = tilesets.iter()
            .find(|&v| map.tileset_file() == v.filename())
            .ok_or(Error::TilemapNotFound)?;

        map.build_tilemap(tileset)?;
    }

    // Check the constraints of the tilemaps
    println!("Checking tilemap constraints...");
    for (i, map) in &mut maps.iter().enumerate() {
        println!("==> {:.2}% done", (i as f32 / maps.len() as f32) * 100f32);
        let res = check_map_constraints(&map)?;

        if !res.is_empty() {
            println!("Found candidate map: {} => {:02X?}", map.name(), res);
        }
    }

    Ok(())
}
