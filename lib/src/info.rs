use std::process::Command;

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
