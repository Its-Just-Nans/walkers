use egui::pos2;
use egui::ColorImage;
use egui::Context;
use egui::Mesh;
use egui::Rect;
use egui::Vec2;
use walkers::sources::Attribution;
use walkers::Texture;
use walkers::TileId;
use walkers::Tiles;

pub struct LocalTiles {
    egui_ctx: Context,
}

impl LocalTiles {
    pub fn new(egui_ctx: Context) -> Self {
        Self { egui_ctx }
    }
}

impl Tiles for LocalTiles {
    fn with_position_and_size(&mut self, _tile_id: TileId, pos: Vec2, size: f64) -> Option<Mesh> {
        let image = ColorImage::example();

        let texture = Texture::from_color_image(image, &self.egui_ctx);
        let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
        Some(texture.mesh_with_uv(pos, size, uv))
    }

    fn attribution(&self) -> Attribution {
        Attribution {
            text: "Local rendering example",
            url: "https://github.com/podusowski/walkers",
            logo_light: None,
            logo_dark: None,
        }
    }

    fn tile_size(&self) -> u32 {
        256
    }
}
