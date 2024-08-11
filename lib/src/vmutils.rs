//! This file contains the funcitons and other utilities used for managing all
//! the virtual machines, `.iso/.img `files and other metadata related to
//! managing all virtual machines.
//!
//! This also includes things such as checksums for the relevant files and other
//! related things.
//!
//! ---

use std::process::Command;

/// This function is used to get the checksum of a specified image file.
///
/// It uses the `md5sum` or `sha256sum` command to get the checksum of the file
/// instead of doing it in  rust since those commands are included in the gnu
/// coreutils and its easier for me since I'm a skid and have skill issues.
///
/// ---
pub fn get_image_checksum(file: &String) {
    let mut cmd = Command::new("sha256sum");
    cmd.arg(file);

    let output = cmd.output().expect("Failed to execute checksum command");
    println!("Checksum {}", String::from_utf8_lossy(&output.stdout));
}


