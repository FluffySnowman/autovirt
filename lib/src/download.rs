use reqwest::blocking::Client;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io::copy;

use std::fs::{self};
use std::io::{self};
use std::path::PathBuf;

use crate::filesystem;

/// DATA DIRECTORY IS NOW IN THE FS.RS FILE AND IS AT ~/.autovirt
// pub const AUTOVIRT_DATA_DIR: &str = "lib/_data/";

/// Hashmap for all supported images
static mut AVAILABLE_IMAGES: Option<HashMap<&'static str, &'static str>> = None;

/// THIS FUNCTION IS NOT USED SINCE NOW ALL THE DATA IS STORED IN A JSON FILE.
/// See filesystem.rs for more info
/// INit available images hashmap
pub fn init_available_images() {
    unsafe {
        if AVAILABLE_IMAGES.is_none() {
            let mut m = HashMap::new();
            m.insert("ubuntu1804", "https://cloud-images.ubuntu.com/releases/18.04/release/ubuntu-18.04-server-cloudimg-amd64.img");
            m.insert("ubuntu2004", "https://cloud-images.ubuntu.com/releases/20.04/release/ubuntu-20.04-server-cloudimg-amd64.img");
            m.insert("ubuntu2204", "https://cloud-images.ubuntu.com/releases/22.04/release/ubuntu-22.04-server-cloudimg-amd64.img");
            AVAILABLE_IMAGES = Some(m);
        }
    }
}

/// Downloads the image for the specified OS/distro to the isos directory in
/// this project. This is subject to change since there will be an option for
/// the user to specify where to download the image or the download(s) will be
/// placed in the user's $HOME or on a system level location that is consistent
/// across machines.
///
/// This takes the name of the distro as an argument and downloads whatever is
/// needed based on which distro matches the name.
///
/// If there is no match there will either be an error message and/or a list of
/// all available images and will/or will default to the ubuntu 22.04 image.
///
pub fn download_vm_image(distro: &String) -> Result<(), Box<dyn Error>> {
    // Get the download link from the JSON config file
    let distro_link = match filesystem::get_value_from_autovirt_json(&format!("images.{}.link", distro)) {
        Some(link) => link.as_str().unwrap_or("").to_string(),
        None => {
            eprintln!("ERROR: Could not find a download link for distro -> {}", distro);
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Distro link not found")));
        }
    };

    // Get the filename for the distro
    let distro_filename = match filesystem::get_value_from_autovirt_json(&format!("images.{}.filename", distro)) {
        Some(filename) => filename.as_str().unwrap_or("").to_string(),
        None => {
            eprintln!("ERROR: Could not find a filename for distro -> {}", distro);
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Distro filename not found")));
        }
    };

    // Construct the full file path
    let data_dir = filesystem::get_autovirt_data_dir().unwrap().join("_data/downloads/");
    fs::create_dir_all(&data_dir)?; // Ensure the download directory exists
    let file_path = data_dir.join(distro_filename);

    // Initialize HTTP client and make the request
    let client = Client::new();
    println!("Downloading image for {} from {}...", distro, distro_link);
    let mut response = client.get(&distro_link).send()?;

    if response.status().is_success() {
        let mut file = File::create(&file_path)?;
        copy(&mut response, &mut file)?;
        println!("Downloaded VM image to -> {}", file_path.to_string_lossy());
    } else {
        eprintln!("ERROR: Failed to download -> {}", response.status());
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("HTTP error -> {}", response.status()),
        )));
    }

    Ok(())
}




// pub fn download_vm_image(distro: &String) -> Result<(), Box<dyn Error>> {
//     init_available_images();

//     let url;

//     // Safely access the global hashmap
//     unsafe {
//         if let Some(images) = &AVAILABLE_IMAGES {
//             if let Some(link) = images.get(distro.as_str()) {
//                 println!("DOWNLOAD LINK FOR {}: {}", distro, link);
//                 url = link;
//             } else {
//                 eprintln!("NO LINK FOR DISTRO FOUND: {}", distro);
//                 return Err(Box::new(std::io::Error::new(
//                     std::io::ErrorKind::NotFound,
//                     "No downlead link found for the specified distro",
//                 )));
//             }
//         } else {
//             eprintln!("AVAILABLE_IMAGES not initialised");
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "AVAILABLE_IMAGES not initialised",
//             )));
//         }
//     }

//     // getting the user's home diretory

//     let file_path = format!("{}-autovirt.img", distro);

//     let client = Client::new();

//     println!("Downloading image for {}...", distro);
//     let mut response = match client.get(url.clone()).send() {
//         Ok(resp) => resp,
//         Err(e) => {
//             eprintln!("ERROR: failed to send request -> {}", e);
//             return Err(Box::new(e));
//         }
//     };

//     if response.status().is_success() {
//         let mut file = match File::create(&file_path) {
//             Ok(f) => f,
//             Err(e) => {
//                 eprintln!("ERROR: failed to create file -> {}", e);
//                 return Err(Box::new(e));
//             }
//         };

//         if let Err(e) = copy(&mut response, &mut file) {
//             eprintln!("ERROR: failed to copy data to file -> {}", e);
//             return Err(Box::new(e));
//         }

//         println!("Downloaded vm file/image to -> {}", file_path);
//     } else {
//         eprintln!("ERROR: failed to download -> {}", response.status());
//         return Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             format!("ERROR: some HTTP error -> {}", response.status()),
//         )));
//     }

//     Ok(())
// }
