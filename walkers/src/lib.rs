#![doc = include_str!("../README.md")]
#![deny(missing_docs, clippy::unwrap_used, rustdoc::broken_intra_doc_links)]

mod center;
mod download;
pub mod extras;
mod io;
mod map;
mod mercator;
pub mod sources;
mod tiles;
mod zoom;

pub use download::{HeaderValue, HttpOptions};
pub use map::{Map, MapMemory, Plugin, Projector};
pub use mercator::{screen_to_position, Position, TileId};
pub use tiles::{HttpTiles, Texture, TextureWithUv, Tiles};
pub use zoom::InvalidZoom;
