use std::fmt::format;
// use std::process::Command;
use std::thread;
use std::time;

use crate::filesystem;

/// The VM sizes (vcpus, ram, disk etc.)
#[derive(Debug)]
pub struct _VMSizes {
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
  - name: AUTOVIRT_USER
    plain_text_passwd: AUTOVIRT_PASS
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
    vm_cpus: &String,
) {
    // print debug info if the user has set the AUTOVIRT_DEBUG env var to 1
    if std::env::var("AUTOVIRT_DEBUG").is_ok() {
        println!("AUTOVIRT DEBUG IS ON");
    }

    // Print vm details with ansi colours
    println!("\x1b[0;32m------ VM Details -----\x1b[0m");
    println!("\x1b[0;32mNAME: \x1b[0m{}", vm_name);
    println!("\x1b[0;32mDISTRO: \x1b[0m{}", vm_dist);
    println!("\x1b[0;32mSIZE: \x1b[0m{}", vm_size);
    println!("\x1b[0;32mUSERNAME: \x1b[0m{}", vm_user);
    println!("\x1b[0;32mPASSWORD: \x1b[0m{}", vm_pass);
    println!("\x1b[0;32mMEMORY: \x1b[0m{}", vm_memory_mb);
    println!("\x1b[0;32mVCPUS: \x1b[0m{}", vm_cpus);
    println!("\x1b[0;32m-----------------------\x1b[0m");

    println!("LOG:: Adding VM metadata to autovirt config...");

    // Adding all the vm details to the autovirt config
    let vm_metadata = serde_json::json!({
        "name": vm_name,
        "distro": vm_dist,
        "size": vm_size,
        "user": vm_user,
        "password": vm_pass,
        "memory_mb": vm_memory_mb,
        "cpus": vm_cpus,
    });

    // insertingt the vm metadat to the autovirt config
    filesystem::insert_value_into_autovirt_json_object(
        &format!("vms.{}", vm_name),
        vm_metadata,
    );

    println!("LOG:: Executing vm startup process in 3 seconds...");
    let startup_wait = time::Duration::from_secs(3);

    thread::sleep(startup_wait);
    println!("\x1b[0;32mLOG:: Creating VM...\x1b[0m");

    // Doing string replaces for the user-data for the cloud init config file.
    let final_user_data = USER_DATA
        .replace("AUTOVIRT_USER", vm_user)
        .replace("AUTOVIRT_PASS", vm_pass)
        .replace("eeeeeeeeeeee", "your mom");

    println!("\x1b[0;32mLOG:: Writing to user-data (cloud-init) file...\x1b[0m");
    // println!("Writing to user-data (cloud-init) file...");

    // Writing the cloud init user data to the user-data file which is served in
    // the imds http server hence used for the creation of the vm with the
    // specified user deets (args to the cli/cmd/function).
    //
    std::fs::write("./lib/src/conf/user-data", final_user_data)
        .expect("failed to write user-data file");

    // print with ansi colors
    println!("\x1b[0;32mLOG:: User-data file written successfully\x1b[0m");

    println!(
        "\x1b[0;32mLOG:: Attempting to create VM disk of size: {}...\x1b[0m",
        vm_size
    );
    // println!("Attempting to create VM disk of size: {}...", vm_size);
    // Resizing the vm to the specified disk size (in the cli args) / creating
    // the disk.
    // Using string formatting because I don't care.

    // FIX:
    // fix vm resize command since it will append more gb's to the disk if
    // the same disk is used
    //
    // TODO:
    // fix this shit bruh
    //

    let disk_size_amount = vm_size.parse::<u32>().unwrap();
    let disk_resize_cmd = format!(
        "qemu-img resize /home/fluffy/.autovirt/_data/downloads/ubuntu-22.04-autovirt-server-cloudimg-amd64.img +{}G",
        disk_size_amount
    );

    println!("\x1b[0;32mLOG:: Resizing disk to {}G...\x1b[0m", vm_size);
    // println!("Resizing disk to {}G...", vm_size);
    let disk_resize_output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&disk_resize_cmd)
        .output()
        .expect("failed to resize disk");

    if disk_resize_output.status.success() {
        println!(
            "\x1b[0;32mLOG:: VM disk resized slccessfully to {}G\x1b[0m",
            vm_size
        );
        // println!("VM disk resized slccessfully to {}G", vm_size);
    } else {
        // print with ansi colors
        eprintln!("\x1b[0;31mERROR:: FAILED TO RESIZE DISK\x1b[0m");
        // eprintln!("FAILED TO RESIZE DISK");
        eprintln!("ERROR:: Command exit code: {}", disk_resize_output.status);
    }

    // Building command to create a vm
    let mut create_vm_cmd = std::process::Command::new("qemu-system-x86_64");
    create_vm_cmd
        .arg("-net")
        .arg("nic")
        .arg("-net")
        .arg("user,hostfwd=tcp::2222-:22") // forwarding ssh to 2222 on host
        .arg("-machine")
        .arg("accel=kvm:tcg")
        .arg("-m")
        .arg(&vm_memory_mb)
        .arg("-nographic")
        .arg("-hda")
        .arg("/home/fluffy/.autovirt/_data/downloads/ubuntu-22.04-autovirt-server-cloudimg-amd64.img")
        .arg("-smbios")
        .arg(format!("type=1,serial=ds=nocloud;s=http://10.0.2.2:8000/"))
        .arg("-serial")
        .arg("pty")
        .arg("-smp")
        .arg(format!("cpus={}", vm_cpus));

    println!("\nNote: Set AUTOVIRT_DEBUG=1 to see the command to be executed\nAlong with other debug info.\n");

    if std::env::var("AUTOVIRT_DEBUG").is_ok() {
        // Printing vm creation command if debug env var is present/is '1'
        println!("INFO:: {:?}", create_vm_cmd);
    }

    let status = create_vm_cmd
        .status()
        .expect("ERROR:: failed to exec vm craeted cmd");

    if status.success() {
        println!("\nLOG:: AutoVirt run success 👍");
        // return;
    } else {
        eprintln!(
            "ERROR:: Something went wrong or something failed to do something with
            \nthe vm\nAUTOVIRT_DEBUG=1 and re-run for more info"
        );
    }
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

// qemu-system-x86_64 \
//     -net nic \
//     -net user,hostfwd=tcp::2222-:22 \
//     -machine accel=kvm:tcg \
//     -m 512 \
//     -nographic \
//     -hda ubuntu-22.04-server-cloudimg-amd64.img \
//     -smbios type=1,serial=ds='nocloud;s=http://10.0.2.2:8000/'

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
