use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use std::fs::OpenOptions;

const DEFAULT_AUTOVIRT_CONFIG_DATA: &str = r#"
{
    "something": "autovirt",
    "version": "0.0.1",
    "images": {
        "ubuntu1804": {
            "link": "https://cloud-images.ubuntu.com/releases/18.04/release/ubuntu-18.04-server-cloudimg-amd64.img",
            "filename": "ubuntu-18.04-autovirt-server-cloudimg-amd64.img"
        },
        "ubuntu2004": {
            "link": "https://cloud-images.ubuntu.com/releases/20.04/release/ubuntu-20.04-server-cloudimg-amd64.img",
            "filename": "ubuntu-20.04-autovirt-server-cloudimg-amd64.img"
        },
        "ubuntu2204": {
            "link": "https://cloud-images.ubuntu.com/releases/22.04/release/ubuntu-22.04-server-cloudimg-amd64.img",
            "filename": "ubuntu-22.04-autovirt-server-cloudimg-amd64.img"
        }
    },
    "downloaded_images": {},
    "vms": {}
}
"#;

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

/// Function to add links to the autovirt.json file so available images can be
/// searched.
///
/// This also adds other things such as the version, other metadata and some
/// other things which will probably change in the future.
///
/// List of things that are added/returned by this to the config file:
///
/// - version               `# autovirt version`
/// - images                `# all the images available for autovirt`
///   - ubuntu2204: link    `# link to distro download`
///   - ubuntu2204: link    `# link to distro download`
/// - other metadata
///
/// Some things may change here and alot of other things will be added in the
/// future.
///
/// The json file will also have extra data for the list of created vm's, the
/// size of vm's and other vm metadata.
///
/// ---
pub fn insert_autovirt_config_data() -> io::Result<()> {
    // let v: Value = serde_json::from_str(DEFAULT_AUTOVIRT_CONFIG_DATA)?;

    if let Some(autovirt_dir) = get_autovirt_data_dir() {
        let json_file_path = autovirt_dir.join("autovirt.json");
        println!("\nINFO:: Path to autovirt.json config file -> {:?}", json_file_path);

        let mut data_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(json_file_path)?;

        data_file.write_all(DEFAULT_AUTOVIRT_CONFIG_DATA.as_bytes())?;
    } else {
        eprintln!("ERROR: something went wrong with insert_autovirt_config_data");
    }

    Ok(())

    // let mut file = File::create(get_autovirt_data_dir());
    // println!("Testing init data: {}, {}", v["version"], v["images"]["ubuntu2204"]["link"]);
}
