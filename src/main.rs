use std::fs::File;

use app::App;
use clap::Parser;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

pub mod app;
pub mod ftc_dashboard;
pub mod ftc_proto;
pub mod gamepad_map;
pub mod input;
pub mod r#match;
pub mod network;
pub mod popup;
pub mod renderers;
pub mod robot;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    export_telemetry: bool,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        File::create("latest.log").unwrap(),
    )])
    .unwrap();

    let args = Args::parse();

    let app = App::new(args).await;

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app.run(terminal).await;
    ratatui::restore();
    result
}
