use simple_server::Server;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::filesystem;

/// Function to start the IDMS server (Instance Metadata Service) for the
/// metadata for a VM.
///
/// Its just a simple httpserverthat serves files from the config directory in
/// this project.
///
/// The cofig directory has cloud-init config files (user-data, meta-data etc.)
/// which may have dynamic parameters based on user specified value.
///
pub fn start_idms_server() {
    let server = Server::new(move |request, mut response| {
        let path = request.uri().path().trim_start_matches('/');
        let path = if path.is_empty() { "index.html" } else { path };

        // Getting the path of the autovirt data directory where the cloud init
        // config files are stored and then serving them

        let autovirt_data_dir = filesystem::get_autovirt_data_dir();
        let autovirt_cloud_init_dir = autovirt_data_dir.unwrap().join("_data/conf");

        let file_path = Path::new(&autovirt_cloud_init_dir).join(path);
        println!("Starting imds server with path -> {:?}", file_path);

        if file_path.is_file() {
            let mut file = File::open(&file_path).expect("file. not found rip");
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).expect("im dyslexic");
            Ok(response.body(contents)?)
        } else {
            let not_found = "404 🫡";
            Ok(response.status(404).body(not_found.as_bytes().to_vec())?)
        }
    });

    // again hardcoded in since I cba  dealing with a million different types
    // for just a string.
    server.listen("0.0.0.0", "8000");
}
