use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

// use std::collections::HashMap;
// use std::fs::{self, File};
// use std::io::{self, Read, Write};
// use std::path::PathBuf;
// use serde_json::{Value, json};

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


// Function to add links to the autovirt.json file so available images can be
// searched.
//
// ---
// pub fn add_vm_links_to_autovirt_json() -> io::Result<()> {
//     let home_dir = env::var("HOME").map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not find $HOME directory"))?;
//     let json_file_path = PathBuf::from(home_dir).join(".autovirt/autovirt.json");

//     let mut file = File::open(&json_file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let mut json_data: Value = serde_json::from_str(&contents)?;

//     let mut images = json_data["images"].as_object_mut().unwrap_or(&mut json!({}).as_object_mut().unwrap());

//     // List of VMs and their links
//     let vm_links = HashMap::from([
//         ("ubuntu1804", "https://cloud-images.ubuntu.com/releases/18.04/release/ubuntu-18.04-server-cloudimg-amd64.img"),
//         ("ubuntu2004", "https://cloud-images.ubuntu.com/releases/20.04/release/ubuntu-20.04-server-cloudimg-amd64.img"),
//         ("ubuntu2204", "https://cloud-images.ubuntu.com/releases/22.04/release/ubuntu-22.04-server-cloudimg-amd64.img"),
//     ]);

//     for (vm, link) in vm_links {
//         images.insert(vm.to_string(), json!(link));
//     }

//     // Write the updated JSON back to the file
//     let mut file = File::create(&json_file_path)?;
//     file.write_all(serde_json::to_string_pretty(&json_data)?.as_bytes())?;

//     Ok(())
// }

