// use std::process::Command;

use crate::filesystem;


/// Function that gets information of a virtual machine by name.
///
/// This simply gets the information of the virtual machine from the
/// `autovirt.json` config file and prints out the plain json.
///
/// ---
pub fn get_vm_info_by_name(vm_name: &String) {
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

// ---
// Gets information of a virtual machine by name.
//
// - Name is passed as an argument (string) then a command (using
// `std::process::command` ) is run executed with the name of the virtual
// machine as the end argument.
//
// - The output of the command is printed here itself and is not returned as
// I'm lazy
//
// ---
//
// # Examples
//
// ```
// let s = "ubuntu-main";
// let vm_name = String::from(s);
// get_vm_info(&vm_name);
// ```
//
// Which will print out something similar (only if the vm name is actually
// there obv) to this:
//
// ```
// Id:             -
// Name:           ubuntu-main
// UUID:           12345667-05b9-40ac-8686-da1234b56694
// OS Type:        hvm
// State:          shut off
// CPU(s):         4
// Max memory:     4194304 KiB
// Used memory:    4194304 KiB
// Persistent:     yes
// Autostart:      disable
// Managed save:   no
// Security model: none
// Security DOI:   0
// ```
//
// If a virtual machine doesn't exist with that name then a  'Failed to get
// VM Information' will be displayed.
//
// ---

// pub fn get_vm_info(vm_name: &String) {
//     let mut vm_information = Command::new("virsh");
//     vm_information.arg("dominfo");
//     let output = vm_information
//         .arg(vm_name)
//         .status()
//         .expect("Failed to get VM Information");
//     println!("{}", output);
// }

// pub fn _show_all_vms() {
//     let mut all_vms = Command::new("virsh list --all");
//     let output = all_vms.status().expect("Command Failed");
//     println!("------ All Virtual Machines ------\n {}", output);
// }
