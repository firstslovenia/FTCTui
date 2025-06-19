use std::fs::File;

use app::App;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

pub mod app;
pub mod ftc_dashboard;
pub mod ftc_proto;
pub mod gamepad_map;
pub mod input;
pub mod r#match;
pub mod network;
pub mod renderers;
pub mod robot;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        File::create("latest.log").unwrap(),
    )])
    .unwrap();

    let app = App::new().await;

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app.run(terminal).await;
    ratatui::restore();
    result
}
