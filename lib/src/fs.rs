use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// Function that gets the data directory for autovirt based on the user's $HOME
/// environment variable.
///
/// If the path exists then it returns the path. If it doesn't then it gives
/// nothing.
///
/// ---
pub fn get_autovirt_data_dir() -> Option<PathBuf> {
    let user_home_path = env::var("HOME");

    match user_home_path {
        Ok(path) => {
            let autovirt_path = PathBuf::from(path).join(".autovirt");
            Some(autovirt_path)
        }
        Err(_) => None,
    }
}

/// Function that creates the autovirt data directory with all the required
/// files.
///
/// It creates:
///
/// - `~/.autovirt/autovirt.json`
/// - `~/.autovirt/_data/ `
///
/// ---
pub fn create_autovirt_data_dir() -> io::Result<()> {
    if let Some(autovirt_dir) = get_autovirt_data_dir() {
        let data_dir = autovirt_dir.join("_data");
        let json_file_path = autovirt_dir.join("autovirt.json");

        // create ~/.autovirt  if it isn't theere
        if !autovirt_dir.exists() {
            fs::create_dir(&autovirt_dir)?;
        }

        // create  ~/.autovirt/autovirt.json if not there
        if !json_file_path.exists() {
            let mut file = fs::File::create(&json_file_path)?;
            // making an empty json file
            file.write_all(b"{}")?;
        }

        // Create  ~/.autovirt/_data  if not exist
        if !data_dir.exists() {
            fs::create_dir(&data_dir)?;
        }

        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "ERROR: COULD NOT FIND USER $HOME DIRECTORY",
        ))
    }
}
