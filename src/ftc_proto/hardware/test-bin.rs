use simplelog::{CombinedLogger, Config, TermLogger};


#[tokio::main]
async fn main() {

    CombinedLogger::init(vec![
        TermLogger::new(log::LevelFilter::Debug, Config::default(), mode, color_choice)
    ])

    let filename = std::env::args().nth(1).unwrap_or("robot-configuration.xml".to_string());
}
