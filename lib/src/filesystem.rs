use serde_json::{Result, Value};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use serde_json::json;

use crate::initdata;


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
        },
        "ubuntu2404": {
            "link": "https://cloud-images.ubuntu.com/releases/24.04/release/ubuntu-24.04-server-cloudimg-amd64.img",
            "filename": "ubuntu-24.04-autovirt-server-cloudimg-amd64.img"
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
/// UPDATE: This function is also now used to add the cloud init config data
/// (sucha seth user-data, vendor-data & meta-data files) to the data directory
/// so that the imds server can use that to do things.
///
/// ---
pub fn insert_autovirt_config_data() -> io::Result<()> {
    // let v: Value = serde_json::from_str(DEFAULT_AUTOVIRT_CONFIG_DATA)?;

    let cloud_init_user_daa = initdata::CLOUD_INIT_USER_DATA;
    let cloud_init_meta_data = initdata::CLOUD_INIT_META_DATA;
    let cloud_init_vendor_data = initdata::CLOUD_INIT_VENDOR_DATA;


    if let Some(autovirt_dir) = get_autovirt_data_dir() {
        let json_file_path = autovirt_dir.join("autovirt.json");
        println!(
            "\nINFO:: Path to autovirt.json config file -> {:?}",
            json_file_path
        );

        let mut data_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(json_file_path)?;

        data_file.write_all(DEFAULT_AUTOVIRT_CONFIG_DATA.as_bytes())?;

        // now adding the data of the cloud init config files to the data
        // directory
        let data_dir = autovirt_dir.join("_data/conf");
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        // writing the cloud init config files to the data directory
        let _file = fs::File::create(data_dir.join("user-data"))?;
        std::fs::write(data_dir.join("user-data"), cloud_init_user_daa)?;
        let _file = fs::File::create(data_dir.join("meta-data"))?;
        std::fs::write(data_dir.join("meta-data"), cloud_init_meta_data)?;
        let _file = fs::File::create(data_dir.join("vendor-data"))?;
        std::fs::write(data_dir.join("vendor-data"), cloud_init_vendor_data)?;

    } else {
        eprintln!("ERROR: something went wrong with insert_autovirt_config_data");
    }

    Ok(())

    // let mut file = File::create(get_autovirt_data_dir());
    // println!("Testing init data: {}, {}", v["version"], v["images"]["ubuntu2204"]["link"]);
}

/// Function to get the path to the autovirt.json file based on the user's $HOME
/// directory.
///
/// This is used in many places to getting the path of the autovirt.json config
/// file and reading data from it to get things such as the images' link for
/// downloads etc.
///
/// ---
pub fn get_autovirt_json_path() -> String {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let json_file_path = PathBuf::from(home_dir)
        .join(".autovirt")
        .join("autovirt.json");
    json_file_path.to_string_lossy().to_string()
}

/// Function to get the value of a specified json key from the autovirt config
/// file such as images.ubuntu2204.link which would return the link of the
/// ubuntu2204 disro for downloads.
///
/// This funciton is used throughout the project to get things from the autovirt
/// config file so they can be used to get vm info, download more vms, get
/// metadata of existing vm's etc.
///
/// ---
pub fn get_value_from_autovirt_json(key: &str) -> Option<Value> {
    let file_path = get_autovirt_json_path();
    let file_content = fs::read_to_string(file_path).unwrap_or_else(|_| "{}".to_string());
    let v: Value = serde_json::from_str(&file_content).unwrap_or_else(|_| Value::Null);

    key.split('.')
        .fold(Some(&v), |acc, part| acc?.get(part))
        .cloned()
}

/// Function to insert a key/value to the autovirt config file.
///
/// This adds a basic key/value to the autovirt.json file in the autovirt data
/// directory at `$HOME/.autovirt/autovirt.json`
///
/// ---
pub fn insert_value_into_autovirt_json(key: &str, value: &str) {
    let file_path = get_autovirt_json_path();

    // Read the existing JSON content
    let file_content = fs::read_to_string(&file_path).unwrap_or_else(|_| "{}".to_string());
    let mut v: Value = serde_json::from_str(&file_content).unwrap_or_else(|_| json!({}));

    // Split the key by '.' to navigate nested JSON objects
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = &mut v;

    for k in keys.iter().take(keys.len() - 1) {
        if !current[k].is_object() {
            current[k] = json!({});
        }
        current = current.get_mut(k).unwrap();
    }

    // Insert or update the value
    current[keys[keys.len() - 1]] = Value::String(value.to_string());

    // Write the modified JSON back to the file
    fs::write(&file_path, serde_json::to_string_pretty(&v).unwrap()).expect("Failed to write to file");
}


/// Same function as above but for json objects ig since the function above is
/// writing newlines `\n` for some reason.
///
/// ---
pub fn insert_value_into_autovirt_json_object(key: &str, value: Value) {
    let file_path = get_autovirt_json_path();

    let file_content = fs::read_to_string(&file_path).unwrap_or_else(|_| "{}".to_string());
    let mut v: Value = serde_json::from_str(&file_content).unwrap_or_else(|_| json!({}));

    let keys: Vec<&str> = key.split('.').collect();
    let mut current = &mut v;

    for k in keys.iter().take(keys.len() - 1) {
        if !current[k].is_object() {
            current[k] = json!({});
        }
        current = current.get_mut(k).unwrap();
    }

    current[keys[keys.len() - 1]] = value;

    fs::write(&file_path, serde_json::to_string_pretty(&v).unwrap()).expect("Failed to write to file");
}

