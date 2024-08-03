// use std::process::Command;
use std::collections::HashMap;

/// The VM sizes (vcpus, ram, disk etc.)
#[derive(Debug)]
pub struct VMSizes {
    vcpus_num: u32,
    ram_mb: u32,
    disk_gb: u32,
}
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
    // Creating the vm details hashmap (sizes, types, ram amount etc)
    let mut vm_meta_details = HashMap::new();
    vm_meta_details.insert(
        "1G",
        VMSizes {
            vcpus_num: 1,
            ram_mb: 1024,
            disk_gb: 25,
        },
    );

    // testing vm size prints/set/get operations
    match vm_meta_details.get("1G") {
        Some(info) => println!(
            "Virtual machine meta info -> VCPS: {:1} RAM (mb): {} DISK (gb): {}  ",
            info.vcpus_num, info.ram_mb, info.disk_gb
        ),
        None => eprintln!("no vm meta details found for whatever you put in oof"),
    }

    // Printing vm details (since I'll probably forget everythnig even though
    // its on the line right above this)

    println!("\n\n------\tNew Virtual Machine Details\t------\n");
    println!("\tVM Name: {}", vm_name);
    println!("\tVM Dist: {}", vm_dist);
    println!("\tVM Size: {}", vm_size);
    println!("\tVM User: {}", vm_user);
    println!("\tVM Pass: {}", vm_pass);

    if vm_size == "1G" {
        println!("vm size is 1g");
    } else {
        println!("vm size not found");
    }


    // Building command to create a vm
}
