// use std::process::Command;
use std::{collections::HashMap, fmt::write};
use std::thread;
use std::time;
use tokio::process::Command;

use reqwest::Certificate;

/// The VM sizes (vcpus, ram, disk etc.)
#[derive(Debug)]
pub struct VMSizes {
    vcpus_num: u32,
    ram_mb: u32,
    disk_gb: u32,
}


/// The user-data cloud init config file (uses interpolation/regex something
/// else) to add the user specified details- like password, username, ssh key
/// etc.
///
pub const USER_DATA: &str = r#"
#cloud-config
users:
  - name: test
    plain_text_passwd: test
    lock_passwd: false
    sudo: ALL=(ALL) NOPASSWD:ALL
    groups: sudo
    shell: /bin/bash
    ssh_import_id: None
    ssh_authorized_keys:
      - ssh-rsa eeeeeeeeeeee

chpasswd:
  expire: false
"#;


// ---
// Install script (bash command) for installing the vm (copy pasted from my
// zsh history
//
// ```bash
// sudo virt-install --name=ubuntutest \
// --vcpus=4 \
// --memory=4096 \
// --location 'http://archive.ubuntu.com/ubuntu/dists/bionic/main/installer-amd64/' \
// --virt-type=kvm \
// --disk size=12 \
// --os-variant=ubuntu18.04
// ````
//
// Install script with cloud init
//
// ```
// qemu-system-x86_64 \
//     -net nic \
//     -net user,hostfwd=tcp::2222-:22 \
//     -machine accel=kvm:tcg \
//     -m 512 \
//     -nographic \
//     -hda ubuntu-22.04-server-cloudimg-amd64.img \
//     -smbios type=1,serial=ds='nocloud;s=http://10.0.2.2:8000/'
// ```
//
// ---

/// Creates a new virtual machine based on the given parameters.
/// This takes the vm name, distro, size, username, password etc. and may even
/// take the path of an ssh key later on as the project progress.s
///
/// This function builds a command and executes it if everythnig is valid.
///
/// Function usage and the end result of the command constructed:
///
/// ```rust
/// scripts::create_new_vm("new vm", "ubuntu2204", "1G", "fluffy", "verystrongpassword");
/// ```
///
pub fn create_new_vm(
    vm_name: &String,
    vm_dist: &String,
    vm_size: &String,
    vm_user: &String,
    vm_pass: &String,
    vm_memory_mb: &String,
) {
    // Creating the vm details hashmap (sizes, types, ram amount etc)
    // let mut vm_meta_details = HashMap::new();
    // vm_meta_details.insert(
    //     "1G",
    //     VMSizes {
    //         vcpus_num: 1,
    //         ram_mb: 1024,
    //         disk_gb: 25,
    //     },
    // );

    // let mut _vcpus_cmd_arg = "";

    // testing vm size prints/set/get operations
    // match vm_meta_details.get("1G") {
    //     Some(info) => {
    //         println!(
    //             "Virtual machine meta info -> VCPS: {:1} RAM (mb): {} DISK (gb): {}  ",
    //             info.vcpus_num, info.ram_mb, info.disk_gb
    //         );
    //         let vcpu_printf = std::fmt::format(format_args!("--vcpus={}", info.vcpus_num));
    //         println!("{}", vcpu_printf);
    //     }
    //     None => eprintln!("no vm meta details found for whatever you put in oof"),
    // }

    // Printing vm details (since I'll probably forget everythnig even though
    // its on the line right above this)

    println!("\n\n------\tNew Virtual Machine Details\t------\n");
    println!("\tVM Name: {}", vm_name);
    println!("\tVM Dist: {}", vm_dist);
    println!("\tVM Size: {}", vm_size);
    println!("\tVM User: {}", vm_user);
    println!("\tVM Pass: {}", vm_pass);
    println!("\tUser specified memory (mb): {}", vm_memory_mb);

    // if vm_size == "1G" {
    //     println!("vm size is 1g");
    // } else {
    //     println!("vm size not found");
    // }

    println!("Executing vm startup process in 3 seconds...");
    let startup_wait = time::Duration::from_secs(3);

    thread::sleep(startup_wait);
    println!("Writing to user-data file");
    std::fs::write("./lib/src/conf/user-data", USER_DATA).expect("failed to write user-data file");

    // Resizing the vm to the specified disk size (in the cli args) / creating
    // the disk.
    // Using string formatting because I don't care.
    let disk_size_amount = vm_size.parse::<u32>().unwrap();
    let disk_resize_cmd = format!("qemu-img resize ./lib/iso_downloads/ubuntu-22.04-server-cloudimg-amd64.img +{}G", disk_size_amount);

    let disk_resize_output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&disk_resize_cmd)
        .output()
        .expect("failed to resize disk");

    println!("{}", disk_resize_output.status);

    // Building command to create a vm

    let mut create_vm_cmd = std::process::Command::new("qemu-system-x86_64");
    create_vm_cmd
        .arg("-net")
        .arg("nic")
        .arg("-net")
        .arg("user,hostfwd=tcp::2222-:22")
        .arg("-machine")
        .arg("accel=kvm:tcg")
        .arg("-m")
        .arg(&vm_memory_mb)
        .arg("-nographic")
        .arg("-hda")
        .arg("./lib/iso_downloads/ubuntu-22.04-server-cloudimg-amd64.img")
        .arg("-smbios")
        .arg(format!("type=1,serial=ds=nocloud;s=http://10.0.2.2:8000/"))
        .arg("-serial")
        .arg("pty");

    // printng cmd cos I probably messed it up
    println!("{:?}", create_vm_cmd);

    let status = create_vm_cmd.status().expect("failed to exec vm craeted cmd");

    if status.success() {
        println!("vm started yeet");
    } else {
        eprintln!("failed to start vm (cloud-init with qemu)");
    }

    // .arg("-net user,hostfwd::2222-:22") // networking to forward ssh-> 2222
    // .arg("-machine accel=kvm:tcg") // kvm accelaration
    // .arg("-m")  // amount of memory for the vm
    // .arg(vm_memory_mb)  // user specified memory (mb)
    // .arg("-nographic")  // no gui (runs in the current terminal)
    // .arg("-hda ./lib/iso_downloads/ubuntu-22.04-server-cloudimg-amd64.img")
    // .arg("-smbios type=1,serial=ds='nocloud;s=http://10.0.2.2:8000/'");

    // let vm_creation_output = create_vm_cmd
    //     .status()
    //     .expect("failed to create vm (cloud-init via qemu-system-x86_64)");
    // println!("{}", vm_creation_output);
}

// qemu-system-x86_64 \
//     -net nic \
//     -net user,hostfwd=tcp::2222-:22 \
//     -machine accel=kvm:tcg \
//     -m 512 \
//     -nographic \
//     -hda ubuntu-22.04-server-cloudimg-amd64.img \
//     -smbios type=1,serial=ds='nocloud;s=http://10.0.2.2:8000/'
