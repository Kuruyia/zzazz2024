use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    RegexError(regex::Error),
    MapBorderBlockConversionFailed,
    MapAttributesNotFound,
    MapConstNotFound,
    MapSizeConversionFailed,
    TilemapNotFound,
    BlockNotFound,
}