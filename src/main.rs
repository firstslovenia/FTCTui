use std::fs::File;

use app::App;
use clap::Parser;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};

use crate::ftc_proto::hardware::document::{try_parse_xml_document, write_xml_document};

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

#[cfg(target_os = "linux")]
pub mod tty;

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

    /// If provided, skips the check for whether we are in a terminal (on Linux).
    ///
    /// Does nothing on Windows.
    ///
    /// You should likely only use this if the tty check doesn't work
    #[arg(long, default_value_t = false)]
    skip_tty_check: bool,

    /// If provided, runs the experimental configuration test (instead of the actual program)
    #[arg(long, default_value_t = false)]
    experimental_configuration_things: bool,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    if let Some(level) = args.log_level.clone()
        && !args.experimental_configuration_things
    {
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

    if args.experimental_configuration_things {
        CombinedLogger::init(vec![
            WriteLogger::new(
                LevelFilter::Trace,
                Config::default(),
                File::create("ftctui.log").unwrap(),
            ),
            TermLogger::new(
                LevelFilter::Trace,
                Config::default(),
                TerminalMode::Mixed,
                simplelog::ColorChoice::Auto,
            ),
        ])
        .unwrap();

        let robot_config = crate::robot::Robot::new_fake().configuration_types.unwrap();

        let original = r#"<?xml version='1.0' encoding='UTF-8' standalone='yes' ?>
<Robot>
    <LynxUsbDevice name="Control Hub Portal" serialNumber="(embedded)" parentModuleAddress="173">
        <LynxModule name="Control Hub" port="173">
            <RevRoboticsUltraplanetaryHDHexMotor name="leftForward" port="0" />
            <RevRoboticsUltraplanetaryHDHexMotor name="frontSideways" port="1" />
            <RevRoboticsUltraplanetaryHDHexMotor name="rightForward" port="2" />
            <RevRoboticsUltraplanetaryHDHexMotor name="backSideways" port="3" />
            <ControlHubImuBHI260AP name="imu" port="0" bus="0" />
            <ModernRoboticsI2cCompassSensor name="Compass" port="1" bus="0" />
        </LynxModule>
        <LynxModule name="Expansion Hub 2" port="2">
            <RevRoboticsUltraplanetaryHDHexMotor name="lifter1" port="0" />
            <RevRoboticsUltraplanetaryHDHexMotor name="lifter2" port="1" />
        </LynxModule>
    </LynxUsbDevice>
</Robot>
"#;

        let start = std::time::Instant::now();
        let a = try_parse_xml_document(original.to_string(), &robot_config).unwrap();
        let parsed = std::time::Instant::now();

        let took_parse = parsed - start;

        log::info!("{}", original);

        log::info!("{:?}", a);

        let start_2 = std::time::Instant::now();
        let b = write_xml_document(&a).unwrap();
        let written = std::time::Instant::now();

        let took_write = written - start_2;

        log::info!("{}", b);
        log::info!("Parsing took {:?}, writing {:?}", took_parse, took_write);
        return Ok(());
    }

    cfg_if::cfg_if! {
       if #[cfg(target_os = "linux")] {
           if !args.skip_tty_check {
               if !tty::is_a_tty() {
                   tty::try_run_in_terminal();
               }
           }
       }
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
