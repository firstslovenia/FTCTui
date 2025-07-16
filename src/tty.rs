//! Linux specific code to run a terminal if we aren't in one
//!
//! so we avoid just running the .exe and rendering into a void

use std::{os::unix::process::CommandExt, process::Command};

/// As a last ditch effort, tries to run our process inside of any terminal that might exist
pub fn try_run_in_terminal() {
    let mut args = vec!["-e".to_string()];
    args.append(&mut std::env::args().collect::<Vec<String>>());
	 args.push("--skip-tty-check".to_string());

    let terminals = ["alacritty", "konsole", "mlterm", "st", "xterm", "lxterm"];

    for terminal in terminals {
        if command_exists(terminal) {
            let _ = Command::new(terminal).args(&args).exec();
        }
    }

    // Some terminals with a special syntax
    args[0] = "--".to_string();

    if command_exists("gnome-terminal") {
        let _ = Command::new("gnome-terminal").args(&args).exec();
    }

    if command_exists("mate-terminal") {
        let _ = Command::new("mate-terminal").args(&args).exec();
    }

    // no extra argument, just after the others
    args.remove(0);

    if command_exists("kitty") {
        let _ = Command::new("kitty").args(&args).exec();
    }
}

/// Returns whether we are inside of a tty, basically inside an actual terminal
///
/// Call the GNU coreutils utility tty -> should be on all systems
pub fn is_a_tty() -> bool {
    match Command::new("tty").arg("-s").status() {
        Ok(status) => return status.code() == Some(0),
        Err(e) => {
            log::error!("Failed to determine whether we are actually in a terminal.");
            log::error!("Error: {}", e);
            log::error!("Please run (inside a terminal) with the --skip-tty-check flag.");

            println!("Failed to determine whether we are actually in a terminal.");
            println!("If you can see this, please run with the --skip-tty-check flag.");
            std::process::exit(13);
        }
    }
}

/// Returns whether a command exists
///
/// used to check which terminals exist, if we aren't inside one
pub fn command_exists(command: &str) -> bool {
    match Command::new("which").arg("-s").arg(command).status() {
        Ok(status) => return status.code() == Some(0),
        Err(e) => {
            log::error!("Failed to invoke 'which' to check which terminals you have.");
            log::error!("Error: {}", e);
            log::error!("Your installation is actually impressive.");

            std::process::exit(14);
        }
    }
}
