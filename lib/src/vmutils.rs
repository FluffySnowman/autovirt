//! This file contains the funcitons and other utilities used for managing all
//! the virtual machines, `.iso/.img `files and other metadata related to
//! managing all virtual machines.
//!
//! This also includes things such as checksums for the relevant files, deleting
//! image vm's and other related things.
//!
//! ---

use std::fmt::format;
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


/// Shows available VM images (using the autovirt json config file)
pub fn show_available_images() {
    let autovirt_json_path = filesystem::get_autovirt_json_path();
    let autovirt_config = fs::read_to_string(&autovirt_json_path)
        .and_then(|content| serde_json::from_str::<Value>(&content).map_err(Into::into))
        .expect("ERROR: Failed to read autovirt.json");

    if let Some(images) = autovirt_config.get("images").and_then(|v| v.as_object()) {
        // loop through the images and only print out the images and nothing
        // under the level of the images key
        for (image_name, _) in images {
            println!("- {}", image_name.color("green"));
        }
    } else {
        println!("No images found.");
    }
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

    // prompt the user to confirm to delete the vm
    println!("Are you sure you want to delete the VM: {}? (yes please/N)", vm_name);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read user input");

    if user_input.trim() != "yes please" {
        println!("!!! ABORTING VM DELETION !!!");
        return;
    }

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

/// Fnuction to resize vm based on the name, disk size, new memory  size and
/// new amount of cpus.
///
/// ---
pub fn resize_vm(
    vm_name: &String,
    vm_disk_resize_args: &String,
    vm_memory_resize_args: &String,
    vm_cpus_resize_args: &String,
) {
    println!("Resizing VM...");
    println!("First resizing the disk...");
    println!("If 0/none provided for the disk then it will stay the same.");
    println!("The rest of the VM (cpus, memory, etc) will be simply updated\nin the autovirt.json file and the vm will have to be stopped and started \nagain for the changes to take into effect.");


    println!("Proceed? (yes please/N)");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read user input");

    if user_input.trim() != "yes please" {
        println!("!!! ABORTING !!!");
        std::process::exit(1);
    }

    // Getting  the vm image path from the config file to resize the disk
    let vm_image_path = filesystem::get_value_from_autovirt_json(&format!("vms.{}.image_path", vm_name))
        .and_then(|v| v.as_str().map(String::from))
        .expect("ERROR: Could not find image path for specified VM");

    println!("INFO:: VM Image Path: {}", vm_image_path);

    // If the vm disk resize args are not 0 or none then resize the command
    // should be run even (with 0 as an arg) and the vm size in the config file
    // will stay the same. If not 0 then the vm size will be updated in the
    // config file.

    let vm_disk_size_formatted = format!("+{}G", vm_disk_resize_args);

    // if vm_disk_resize_args != "+0" && vm_disk_resize_args != "none" {
        let mut cmd = Command::new("qemu-img");
        cmd.arg("resize");
        cmd.arg(&vm_image_path);
        cmd.arg(&vm_disk_size_formatted);

        let output = cmd.output().expect("Failed to execute disk resize command");
        if output.status.success() {
            println!("LOG:: Disk resized successfully.");
        } else {
            eprintln!("ERROR: Failed to resize disk -> {}", String::from_utf8_lossy(&output.stderr));
        }
        // println!("Disk resize output: {}", output.to_);
    // }

    // Updating the autovirt.json file with the new vm size, memory and CPUs
    let autovirt_json_path = filesystem::get_autovirt_json_path();
    let mut autovirt_config = fs::read_to_string(&autovirt_json_path)
        .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).map_err(Into::into))
        .expect("ERROR: Failed to read autovirt.json config file");

    if let Some(vms) = autovirt_config.get_mut("vms").and_then(|v| v.as_object_mut()) {
        if let Some(vm_data) = vms.get_mut(vm_name) {

            // update memory
            if vm_memory_resize_args != "0" && vm_memory_resize_args != "none" {
                vm_data["memory_mb"] = serde_json::Value::String(vm_memory_resize_args.clone());
            }

            // update cpusu
            if vm_cpus_resize_args != "0" && vm_cpus_resize_args != "none" {
                vm_data["cpus"] = serde_json::Value::String(vm_cpus_resize_args.clone());
            }

            // update disk size
            if vm_disk_size_formatted != "+0" && vm_disk_size_formatted != "none" {
                // add the original disk size to the new disk size to get the
                // new disk size
                let new_disk_size = vm_data.get("size").and_then(Value::as_str).unwrap_or("0");
                let new_disk_size_int: i32 = new_disk_size.parse().unwrap();
                let vm_disk_resize_args_int: i32 = vm_disk_resize_args.parse().unwrap();
                let new_disk_size_final = new_disk_size_int + vm_disk_resize_args_int;
                vm_data["size"] = serde_json::Value::String(new_disk_size_final.to_string());
            }

            // Updating the autovirt.json file with the new shit
            fs::write(
                &autovirt_json_path,
                serde_json::to_string_pretty(&autovirt_config).expect("ERROR: Failed to jsonifyyy updated config"),
            ).expect("ERROR: Failed to write updated autovirt.json conf file");
            println!("LOG:: VM resized in autovirt.json conf file -> {}", vm_name);
        } else {
            eprintln!("ERROR: VM entry not found in autovirt.json conifig file -> {}", vm_name);
        }
    } else {
        eprintln!("ERROR: No VMs found in autovirt.json ? idek whats happening here");
    }

    println!("VM resized successfully.");
}


/// Function to clone  a vm based on the name and new name.
///
/// This will create a duplicate entry in the autovirt json file along with the
/// new name and will update the image path + changes the image name in the path
/// and the name of the vm in the new duplicate entry.
pub fn clone_vm(vm_name: &String, vm_new_name: &String) {
    println!("LOG:: Cloning VM...");

    // getting the vm image path to get the vm to clone etc.
    let vm_image_path = filesystem::get_value_from_autovirt_json(&format!("vms.{}.image_path", vm_name))
        .and_then(|v| v.as_str().map(String::from))
        .expect("ERROR: Could not find image path for specified VM");

    println!("INFO:: Current VM Image path (to be cloned) -> {}", vm_image_path);

    // gettig the autovirt json config file path and config contents
    let autovirt_json_path = filesystem::get_autovirt_json_path();
    let mut autovirt_config = fs::read_to_string(&autovirt_json_path)
        .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).map_err(Into::into))
        .expect("ERROR: Failed to read autovirt.json config file");

    // get the vm data from the config file
    let vm_data = autovirt_config.get("vms").and_then(|v| v.as_object()).unwrap().get(vm_name).unwrap();

    println!("INFO:: VM data to be cloned -> {}", vm_data);
    println!("INFO:: New VM name -> {}", vm_new_name);
    println!("LOG:: Updating autovirt config file...");

    // check if the new vm name already exists in the config file
    if autovirt_config["vms"].get(vm_new_name).is_some() {
        eprintln!("ERROR: VM with the name {} already exists in the config file", vm_new_name);
        std::process::exit(1);
    }

    // update the json config file contents with the new vm name and new vm path
    // with the new vm path having the new vm name instead of the current vm name

    let new_vm_image_path = vm_image_path.replace(vm_name, vm_new_name);
    let new_vm_data = vm_data.clone();
    autovirt_config["vms"][vm_new_name] = new_vm_data;
    autovirt_config["vms"][vm_new_name]["image_path"] = serde_json::Value::String(new_vm_image_path.clone());

    println!("INFO:: New VM data -> {}", autovirt_config["vms"][vm_new_name]);

    // confirmation  prompt for the user to confirm
    println!("Are you sure you want to clone the VM: {} to {}? (yes please/N)", vm_name, vm_new_name);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read user input");

    if user_input.trim() != "yes please" {
        println!("!!! ABORTING VM CLONING !!!");
        std::process::exit(1);
    }

    println!("LOG:: PRoceeding to clone VM...");

    // write the updated config file to the autovirt.config json config file
    fs::write(
        &autovirt_json_path,
        serde_json::to_string_pretty(&autovirt_config).expect("ERROR: Failed to jsonifyyy updated config"),
    ).expect("ERROR: Failed to write updated autovirt.json conf file");


    // copy the vm to the new path
    fs::copy(&vm_image_path, &new_vm_image_path).expect("ERROR: Failed to copy base image to _VMS directory");
    println!("LOG:: VM image cloned to: {:?}", new_vm_image_path);
    println!("LOG:: VM cloned successfully.");

    return;


}

