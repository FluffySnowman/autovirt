    // let url = "https://cloud-images.ubuntu.com/releases/22.04/release/ubuntu-22.04-server-cloudimg-amd64.img";

    // let file_path = "ubuntu-22.04-server-cloudimg-amd64.img";


use std::fs::File;
use std::io::copy;
// use std::io::Write;
use std::error::Error;
use reqwest::blocking::Client;
use std::collections::HashMap;

/// Hsahmap for available images to download/use for the vm's.
pub fn available_images() -> HashMap<&'static str, &'static str> {
    let mut available_images = HashMap::new();
    available_images.insert("ubuntu1804", "https://cloud-images.ubuntu.com/releases/18.04/release/ubuntu-18.04-server-cloudimg-amd64.img");
    available_images.insert("ubuntu2004", "https://cloud-images.ubuntu.com/releases/20.04/release/ubuntu-20.04-server-cloudimg-amd64.img");

    available_images
}

// available_images.insert("ubuntu2204", "https://cloud-images.ubuntu.com/releases/22.04/release/ubuntu-22.04-server-cloudimg-amd64.img");


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
pub fn download_vm_image() -> Result<(), Box<dyn Error>> {

    let url = "https://cloud-images.ubuntu.com/releases/22.04/release/ubuntu-22.04-server-cloudimg-amd64.img";
    let file_path = "ubuntu-22.04-server-cloudimg-amd64.img";

    let client = Client::new();

    let mut response = match client.get(url).send() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("ERROR: failed to send request -> {}", e);
            return Err(Box::new(e));
        }
    };

    if response.status().is_success() {
        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("ERROR: failed to create file -> {}", e);
                return Err(Box::new(e));
            }
        };

        if let Err(e) = copy(&mut response, &mut file) {
            eprintln!("ERROR: failed to copy data to file -> {}", e);
            return Err(Box::new(e));
        }

        println!("Downloaded vm file/image to -> {}", file_path);
    } else {
        eprintln!("ERROR: failed tod ownload -> {}", response.status());
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ERROR: some http error  -> {}", response.status())
        )));
    }

    Ok(())
}

