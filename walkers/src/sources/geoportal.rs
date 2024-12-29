use super::{Attribution, TileSource};
use crate::TextureWithUv;
use crate::TileId;
use egui::{Color32, Mesh, Rect, Vec2};

/// Orthophotomap layer from Poland's Geoportal.
/// <https://www.geoportal.gov.pl/uslugi/usluga-przegladania-wms>
#[derive(Default, Clone)]
pub struct Geoportal;

impl TileSource for Geoportal {
    fn tile_url(&self, tile_id: TileId) -> String {
        format!(
            "https://mapy.geoportal.gov.pl/wss/service/PZGIK/ORTO/WMTS/StandardResolution?\
            &SERVICE=WMTS\
            &REQUEST=GetTile\
            &VERSION=1.0.0\
            &LAYER=ORTOFOTOMAPA\
            &TILEMATRIXSET=EPSG:3857\
            &TILEMATRIX=EPSG:3857:{}\
            &TILEROW={}\
            &TILECOL={}",
            tile_id.zoom, tile_id.y, tile_id.x
        )
    }

    fn attribution(&self) -> Attribution {
        Attribution {
            text: "Główny Urząd Geodezji i Kartografii",
            url: "https://www.geoportal.gov.pl/",
            logo_light: None,
            logo_dark: None,
        }
    }

    fn get_mesh(&self, texture_uv: TextureWithUv, pos: Vec2, size: f64) -> Mesh {
        let rec = Rect::from_min_size(pos.to_pos2(), Vec2::splat(size as f32));
        let mut mesh = texture_uv.texture.get_mesh();
        mesh.add_rect_with_uv(rec, texture_uv.uv, Color32::from_rgb(30, 130, 130));
        mesh
    }
}
