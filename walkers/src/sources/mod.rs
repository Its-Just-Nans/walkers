//! Some common HTTP tile sources. Make sure you follow terms of usage of the particular source.

mod geoportal;
mod mapbox;
mod openstreetmap;

use crate::{mercator::TileId, TextureWithUv};
use egui::{Color32, Mesh, Rect, Vec2};
pub use geoportal::Geoportal;
pub use mapbox::{Mapbox, MapboxStyle};
pub use openstreetmap::OpenStreetMap;

#[derive(Clone)]
pub struct Attribution {
    pub text: &'static str,
    pub url: &'static str,
    pub logo_light: Option<egui::ImageSource<'static>>,
    pub logo_dark: Option<egui::ImageSource<'static>>,
}

/// Remote tile server definition, source for the [`crate::HttpTiles`].
pub trait TileSource {
    fn tile_url(&self, tile_id: TileId) -> String;
    fn attribution(&self) -> Attribution;

    /// Size of each tile, should be a multiple of 256.
    fn tile_size(&self) -> u32 {
        256
    }

    fn max_zoom(&self) -> u8 {
        19
    }

    fn get_mesh(&self, texture_uv: TextureWithUv, pos: Vec2, size: f64) -> Mesh {
        let rec = Rect::from_min_size(pos.to_pos2(), Vec2::splat(size as f32));
        let mut mesh = texture_uv.texture.get_mesh();
        mesh.add_rect_with_uv(rec, texture_uv.uv, Color32::WHITE);
        mesh
    }
}
