// use std::process::Command;

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

pub fn create_new_vm(vm_name: &String) {
    println!("Creating new VM with Name: {}", vm_name);
}
