//! This file is not in use. Most of the functions that are/were in this file
//! are now in the vmutils.rs file or somewhere else.
//!
//! ---

// use std::process::Command;

// use crate::filesystem;


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
