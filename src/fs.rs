use std::path::PathBuf;

/// Finds the path to the config folder
pub fn get_config_folder() -> PathBuf {
    let Some(config_dir) = dirs::config_dir() else {
        log::error!("Failed to find your config directory!");
        std::process::exit(50);
    };

    config_dir.join("ftctui")
}

/// Finds the path to the configuration files folder
pub fn get_configurations_folder() -> PathBuf {
    get_config_folder().join("configurations")
}

/// Tries to create the config folder if it doesn't exist
pub fn try_create_folder_if_not_exists(folder: &PathBuf) -> Result<(), std::io::Error> {
    match std::fs::exists(folder) {
        Ok(e) => {
            if e {
                return Ok(());
            }
        }
        Err(e) => {
            log::error!("Failed to check if {:?} exists! ({})", folder, e);
            return Err(e);
        }
    }

    log::info!("Creating {:?}..", folder);

    std::fs::create_dir_all(folder)
}

/// Runs [try_create_folder_if_not_exists], erroring if we can't find it
pub fn create_folder_if_not_exists(folder: &PathBuf) {
    match try_create_folder_if_not_exists(folder) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to create folder {:?}!", folder);
            println!("Error: {e}");
            std::process::exit(51);
        }
    }
}

/// Returns the path to the config file
pub fn get_config_file() -> PathBuf {
    get_config_folder().join("config.toml")
}

