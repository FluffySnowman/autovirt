//! This file contains the funcitons and other utilities used for managing all
//! the virtual machines, `.iso/.img `files and other metadata related to
//! managing all virtual machines.
//!
//! This also includes things such as checksums for the relevant files, deleting
//! image vm's and other related things.
//!
//! ---

use std::process::Command;
use std::fs;
use std::path::PathBuf;

use crate::filesystem;

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


/// Func to delete the vm image file based on the args passed to this function
/// and also updates the `autovirtt.json` config file with the new data (deosnt
/// include the deleted vm details).
///
/// ---
pub fn delete_vm(vm_name: &String) {

    // get vm img ptah from the conf file
    let vm_image_path = filesystem::get_value_from_autovirt_json(&format!("vms.{}.image_path", vm_name))
        .and_then(|v| v.as_str().map(String::from))
        .expect("ERROR: Could not find image path for specified VM");

    // doing type shit since rust is retarded
    let vm_image_path_buf = PathBuf::from(vm_image_path);

    // Actually deleting the vm .img file
    if vm_image_path_buf.exists() {
        fs::remove_file(&vm_image_path_buf).expect("ERROR: Failed to delete VM image file");
        println!("LOG:: VM img file deleted -> {:?}", vm_image_path_buf);
    } else {
        eprintln!("ERROR: VM image file not found -> {:?}", vm_image_path_buf);
    }

    // removing the vm entry from the autovirt config file
    let autovirt_json_path = filesystem::get_autovirt_json_path();
    let mut autovirt_config = fs::read_to_string(&autovirt_json_path)
        .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).map_err(Into::into))
        .expect("ERROR: Failed to read autovirt.json config file");

    if let Some(vms) = autovirt_config.get_mut("vms").and_then(|v| v.as_object_mut()) {
        if vms.remove(vm_name).is_some() {
            // Updating autovirt.json file with the new shit
            fs::write(
                &autovirt_json_path,
                serde_json::to_string_pretty(&autovirt_config).expect("ERROR: Failed to jsonifyyy updated config"),
            ).expect("ERROR: Failed to write updated autovirt.json conf file");
            println!("LOG:: VM entry deleted from conf file -> {}", vm_name);
        } else {
            eprintln!("ERROR: VM entry not found in autovirt.json conifig file -> {}", vm_name);
        }
    } else {
        eprintln!("ERROR: No VMs found in autovirt.json ? idek whats happening here");
    }
}

