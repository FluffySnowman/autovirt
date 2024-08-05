    // let url = "https://cloud-images.ubuntu.com/releases/22.04/release/ubuntu-22.04-server-cloudimg-amd64.img";

    // let file_path = "ubuntu-22.04-server-cloudimg-amd64.img";


use std::fs::File;
use std::io::copy;
use std::io::Write;
use std::error::Error;
use reqwest::blocking::Client;

pub fn download_os_image() -> Result<(), Box<dyn Error>> {

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

