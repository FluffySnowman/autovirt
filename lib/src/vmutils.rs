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
use serde_json::Value;
use colored::*;

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


/// Function that gets information of a virtual machine by name.
///
/// This simply gets the information of the virtual machine from the
/// `autovirt.json` config file and prints out the plain json.
///
/// If the `raw_output` flag is set to true, it will print out the raw json
/// which can be used with jq or for something else to work with the data
/// programmatically.
///
/// ---
pub fn get_vm_info_by_name(vm_name: &String, raw_output: bool) {

    if raw_output {
        let vm_details_raw_json =  filesystem::get_value_from_autovirt_json(&format!("vms.{}", vm_name));
        println!("{}", vm_details_raw_json.unwrap_or_default());
        return;
    } else {

    // let vm_details_raw_json =  filesystem::get_value_from_autovirt_json(&format!("vms.{}", vm_name));
    // println!("Details for vm -> {}\n{}", vm_name, vm_details_raw_json.unwrap_or_default());
    let vm_info_cpus =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.cpus", vm_name));
    let vm_info_distro =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.distro", vm_name));
    let vm_info_image_path =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.image_path", vm_name));
    let vm_info_memory_mb =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.memory_mb", vm_name));
    let vm_info_name =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.name", vm_name));
    let vm_info_size =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.size", vm_name));
    let vm_info_user =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.user", vm_name));
    let vm_info_password =  filesystem::get_value_from_autovirt_json(&format!("vms.{}.password", vm_name));

    println!("Name: {}", vm_info_name.unwrap_or_default());
    println!("CPUs: {}", vm_info_cpus.unwrap_or_default());
    println!("Distro: {}", vm_info_distro.unwrap_or_default());
    println!("Image Path: {}", vm_info_image_path.unwrap_or_default());
    println!("Memory MB: {}", vm_info_memory_mb.unwrap_or_default());
    println!("Size: {}", vm_info_size.unwrap_or_default());
    println!("User: {}", vm_info_user.unwrap_or_default());
    println!("Password: {}", vm_info_password.unwrap_or_default());
    }
}


/// Function to list all the currently installed vms
///
/// ---
pub fn list_vms() {
    // Load the autovirt.json configuration
    let autovirt_json_path = filesystem::get_autovirt_json_path();
    let autovirt_config = fs::read_to_string(&autovirt_json_path)
        .and_then(|content| serde_json::from_str::<Value>(&content).map_err(Into::into))
        .expect("ERROR: Failed to read autovirt.json");

    println!("{}", ".".color("white"));

    if let Some(vms) = autovirt_config.get("vms").and_then(|v| v.as_object()) {
        for (vm_name, vm_data) in vms {
            let distro = vm_data.get("distro").and_then(Value::as_str).unwrap_or("Unknown distro");
            let size = vm_data.get("size").and_then(Value::as_str).unwrap_or("Unknown size");
            let memory_mb = vm_data.get("memory_mb").and_then(Value::as_str).unwrap_or("Unknown memory");
            let vcpus = vm_data.get("cpus").and_then(Value::as_str).unwrap_or("Unknown memory");

            println!("{}", format!("├── {}", vm_name).color("green"));
            println!("{}{}", "│   ├── DISTRO: ".color("white"), distro.color("magenta"));
            println!("{}{}{}", "│   ├── SIZE: ".color("white"), size.color("cyan"), " G".color("cyan"));
            println!("{}{}{}", "│   └── MEMORY: ".color("white"), memory_mb.color("yellow"), " Mb".color("yellow"));
            println!("{}{}{}", "│   └── CPUS: ".color("white"), vcpus.color("green"), " vCPUs".color("green"));
            // println!("{}{}", "│   └── MEMORY: ".color("white"), memory_mb.truecolor(63, 00, 189));
        }
        println!("{}", "└── (End of VMs)".color("white"));
    } else {
        println!("{}", "No VMs found.".color("red"));
    }

}


// obsolete code
    // println!(".");
    // // Check if "vms" exists in the configuration
    // if let Some(vms) = autovirt_config.get("vms").and_then(|v| v.as_object()) {
    //     for (vm_name, vm_data) in vms {
    //         let distro = vm_data.get("distro").and_then(Value::as_str).unwrap_or("Unknown distro");
    //         let size = vm_data.get("size").and_then(Value::as_str).unwrap_or("Unknown distro");
    //         let memory_mb = vm_data.get("memory_mb").and_then(Value::as_str).unwrap_or("Unknown distro");
    //         // println!("- Name{}, Distro: {}", vm_name, distro);
    //         // println!("│> {}", vm_name.color("green"));
    //         // println!("{} {} {}", "├──", " DIST:".color("cyan"), distro.color("cyan"));
    //         // println!("├── SIZE: {}G", size);
    //         // println!("{} {} {}G", "├──  ".color("cyan"), "SIZE: ", size.color("wh"));
    //         // println!("{}{}{}{}", "├──  ".color("white"), "MEM: ", memory_mb.color("magenta"), "mb".color("green"));
    //         // println!("├──  MEM:  {}mb", memory_mb);
    //         // println!("{}{}{}{}{}{}{}{}", "├──  ".color("white"), "NAME: ", vm_name.color("green"), "  DIST: ", distro.color("cyan"), "  SIZE: ", size.color("wh"), "G");
    //         // printing all the vm details in a tree-like structure with good
    //         // colours similar to the tree command on linux with the details on
    //         // new lines of the tree

    //     }
    // } else {
    //     println!("No VMs found.");
    // }


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

