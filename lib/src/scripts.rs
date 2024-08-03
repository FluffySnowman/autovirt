// use std::process::Command;

/// ---
/// Install script (bash command) for installing the vm (copy pasted from my
/// zsh history
///
/// ```bash
/// sudo virt-install --name=ubuntutest \
/// --vcpus=4 \
/// --memory=4096 \
/// --location 'http://archive.ubuntu.com/ubuntu/dists/bionic/main/installer-amd64/' \
/// --virt-type=kvm \
/// --disk size=12 \
/// --os-variant=ubuntu18.04
/// ````
///
/// *bruh*
///
/// ---

pub fn create_new_vm(
    vm_name: &String,
    vm_dist: &String,
    vm_size: &String,
    vm_user: &String,
    vm_pass: &String,
) {
    // Building command to create a vm
    println!("\n\n------\tNew Virtual Machine Details\t------\n");
    println!("\tVM Name: {}", vm_name);
    println!("\tVM Dist: {}", vm_dist);
    println!("\tVM Size: {}", vm_size);
    println!("\tVM User: {}", vm_user);
    println!("\tVM Pass: {}", vm_pass);
}
