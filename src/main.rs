use std::fs::File;

use app::App;
use clap::Parser;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

pub mod app;
pub mod command;
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
pub struct Args {
    /// Whether or not to export receieved telemetry packets as a file called telemetry_log.json
    ///
    /// More info on how this is structured or how to use the dumped data can be found on the
    /// Github readme.
    #[arg(short, long, default_value_t = false)]
    export_telemetry: bool,

    /// The level of messages to log at.
    ///
    /// By default, ftctui does not create logs.
    /// If this is set, will create a log file at ftctui.log
    ///
    /// The possible levels are error, warn, info, debug and trace.
    ///
    /// When submitting a bug report, please use trace if possible.
    #[arg(short, long)]
    log_level: Option<String>,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    if let Some(level) = args.log_level.clone() {
        let level_filter = match level.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => {
                println!("Invalid log level {:?}!", level.to_lowercase());
                println!("Please see --help for possible values.");
                return Ok(());
            }
        };

        CombinedLogger::init(vec![WriteLogger::new(
            level_filter,
            Config::default(),
            File::create("ftctui.log").unwrap(),
        )])
        .unwrap();
    }

    let app = App::new(args).await;

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app.run(terminal).await;
    ratatui::restore();

    if result.is_ok() {
        std::process::exit(0);
    }

    result
}
