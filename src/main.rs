use std::{fs::File, time::Duration};

use app::App;
use r#match::MatchSFXHandler;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, WriteLogger};

pub mod app;
pub mod ftc_dashboard;
pub mod ftc_proto;
pub mod input;
pub mod r#match;
pub mod network;
pub mod renderers;
pub mod robot;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("log.log").unwrap(),
        ),
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
    ])
    .unwrap();

    let current_match = r#match::Match {
        start: std::time::Instant::now(),
    };

    let (sender, receiver) = tokio::sync::broadcast::channel(2);

    let mut sfx_handler = MatchSFXHandler {
        receiver,
        thread: None,
    };

    tokio::spawn(async move { sfx_handler.handler_thread().await });

    sender.send(Some(current_match)).unwrap();

    loop {
        let phase = current_match.phase();
        let timer = current_match.remaining_in_phase();

        log::info!("{:?}, {:.1?} left", phase, timer);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    return Ok(());

    let app = App::new().await;

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app.run(terminal).await;
    ratatui::restore();
    result
}
