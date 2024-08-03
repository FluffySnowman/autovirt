use std::process::Command;

/// ---
/// Gets information of a virtual machine by name.
///
/// * Name is passed as an argument (string) then a command (using
/// `std::process::command` ) is run executed with the name of the virtual
/// machine as the end argument.
///
/// * The output of the command is printed here itself and is not returned as
/// I'm lazy
///
/// # Examples
/// ```
/// let s = "ubuntu-main";
/// let vm_name = String::from(s);
/// get_vm_info(&vm_name);
/// ```
///
/// ---

pub fn get_vm_info(vm_name: &String) {
    let mut vm_information = Command::new("virsh");
    vm_information.arg("dominfo");
    let output = vm_information.arg(vm_name).status().expect("Failed to get VM Information");
    println!("{}", output);
}

pub fn show_all_vms() {
    let mut all_vms = Command::new("virsh list --all");
    let output = all_vms.status().expect("Command Failed");
    println!("------ All Virtual Machines ------\n {}", output);
}
