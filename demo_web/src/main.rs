use demo::Provider;
use walkers::sources::{Attribution, TileSource};
use walkers::{TileId, Tiles};

pub struct CustomSource {}

impl CustomSource {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_provider_name() -> Provider {
        Provider::Custom("Humanitarian OpenStreepMap".to_owned())
    }
}

impl TileSource for CustomSource {
    fn tile_url(&self, tile_id: TileId) -> String {
        format!(
            "https://tile-b.openstreetmap.fr/hot/{}/{}/{}.png",
            tile_id.zoom, tile_id.x, tile_id.y
        )
    }

    fn attribution(&self) -> Attribution {
        Attribution {
            text: "Humanitarian OpenStreetMap contributors",
            url: "https://www.openstreetmap.org/copyright",
            logo_light: None,
            logo_dark: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    Box::new(demo::MyApp::new(cc.egui_ctx.clone()).with_provider(
                        CustomSource::get_provider_name(),
                        Box::new(Tiles::new(CustomSource::new(), cc.egui_ctx.to_owned())),
                    ))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This demo is only meant to be compiled for WASM.");
}
