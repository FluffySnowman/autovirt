use std::fmt::format;
// use std::process::Command;
use std::thread;
use std::time;

use crate::filesystem;
use crate::initdata;

/// The VM sizes (vcpus, ram, disk etc.)
#[derive(Debug)]
pub struct _VMSizes {
    vcpus_num: u32,
    ram_mb: u32,
    disk_gb: u32,
}


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
use std::fs;
use std::process::Command;
use std::time::Duration;

pub fn create_new_vm(
    vm_name: &String,
    vm_dist: &String,
    vm_size: &String,
    vm_user: &String,
    vm_pass: &String,
    vm_memory_mb: &String,
    vm_cpus: &String,
    vm_ssh_key: &String,
) {
    // Print debug info if the user has set the AUTOVIRT_DEBUG env var to 1
    if std::env::var("AUTOVIRT_DEBUG").is_ok() {
        println!("AUTOVIRT DEBUG IS ON");
    }

    // Print VM details with ANSI colors
    println!("\x1b[0;32m------ VM Details -----\x1b[0m");
    println!("\x1b[0;32mNAME: \x1b[0m{}", vm_name);
    println!("\x1b[0;32mDISTRO: \x1b[0m{}", vm_dist);
    println!("\x1b[0;32mSIZE: \x1b[0m{}", vm_size);
    println!("\x1b[0;32mUSERNAME: \x1b[0m{}", vm_user);
    println!("\x1b[0;32mPASSWORD: \x1b[0m{}", vm_pass);
    println!("\x1b[0;32mMEMORY: \x1b[0m{}", vm_memory_mb);
    println!("\x1b[0;32mVCPUS: \x1b[0m{}", vm_cpus);
    println!("\x1b[0;32mSSH KEY: \x1b[0m{}", vm_ssh_key);
    println!("\x1b[0;32m-----------------------\x1b[0m");

    // Fetch the filename for the specified distro
    let distro_filename = filesystem::get_value_from_autovirt_json(&format!("images.{}.filename", vm_dist))
        .and_then(|v| v.as_str().map(String::from))
        .expect("ERROR: Could not find the filename for the specified distro");

    // Construct the full path for the VM image to be created in the _VMS directory
    let vms_dir = filesystem::get_autovirt_data_dir().unwrap().join("_VMS");
    fs::create_dir_all(&vms_dir).expect("ERROR: Could not create _VMS directory");

    let vm_image_name = format!("{}-autovirt-{}", vm_name, distro_filename);
    let vm_image_path = vms_dir.join(&vm_image_name);

    // Copy the base distro image to the _VMS directory with the new VM name
    let base_image_path = filesystem::get_autovirt_data_dir().unwrap().join("_data/downloads").join(distro_filename);
    fs::copy(&base_image_path, &vm_image_path).expect("ERROR: Failed to copy base image to _VMS directory");

    println!("LOG:: VM image copied to: {:?}", vm_image_path);

    // Add the VM details to the autovirt config, including the VM image path
    let vm_metadata = serde_json::json!({
        "name": vm_name,
        "distro": vm_dist,
        "size": vm_size,
        "user": vm_user,
        "password": vm_pass,
        "memory_mb": vm_memory_mb,
        "cpus": vm_cpus,
        "image_path": vm_image_path.to_string_lossy(),
    });

    filesystem::insert_value_into_autovirt_json_object(
        &format!("vms.{}", vm_name),
        vm_metadata,
    );

    println!("LOG:: Executing VM startup process in 3 seconds...");
    thread::sleep(Duration::from_secs(3));
    println!("\x1b[0;32mLOG:: Creating VM...\x1b[0m");

    // Reading the contents of the ssh key file specified by the user
    let ssh_key_content = fs::read_to_string(vm_ssh_key).expect("ERROR: failed to read ssh key file contents");

    // Doing string replaces for the user-data for the cloud init config file.
    let final_user_data = initdata::CLOUD_INIT_USER_DATA
        .replace("AUTOVIRT_USER", vm_user)
        .replace("AUTOVIRT_PASS", vm_pass)
        .replace("AUTOVIRT_SSH_KEY", &ssh_key_content);

    let autovirt_data_dir = filesystem::get_autovirt_data_dir();
    let autovirt_data_dir_cloud_init_user_data_file = autovirt_data_dir.unwrap().join("_data/conf/user-data");

    println!("\x1b[0;32mLOG:: Writing to user-data (cloud-init) file...\x1b[0m");
    std::fs::write(autovirt_data_dir_cloud_init_user_data_file, final_user_data)
        .expect("ERROR: Failed to write user-data file");
    println!("\x1b[0;32mLOG:: User-data file written successfully\x1b[0m");

    // Resizing the VM disk to the specified size (in the cli args)
    let disk_size_amount = vm_size.parse::<u32>().unwrap();
    let disk_resize_cmd = format!(
        "qemu-img resize {:?} +{}G",
        vm_image_path, disk_size_amount
    );

    println!("\x1b[0;32mLOG:: Resizing disk to {}G...\x1b[0m", vm_size);
    let disk_resize_output = Command::new("sh")
        .arg("-c")
        .arg(&disk_resize_cmd)
        .output()
        .expect("ERROR: Failed to resize disk");

    if disk_resize_output.status.success() {
        println!("\x1b[0;32mLOG:: VM disk resized successfully to {}G\x1b[0m", vm_size);
    } else {
        eprintln!("\x1b[0;31mERROR:: FAILED TO RESIZE DISK\x1b[0m");
        eprintln!("ERROR:: Command exit code: {}", disk_resize_output.status);
    }

    // Building command to create a VM
    let mut create_vm_cmd = Command::new("qemu-system-x86_64");
    create_vm_cmd
        .arg("-net")
        .arg("nic")
        .arg("-net")
        .arg("user,hostfwd=tcp::2222-:22") // forwarding SSH to 2222 on host
        .arg("-machine")
        .arg("accel=kvm:tcg")
        .arg("-m")
        .arg(vm_memory_mb)
        .arg("-nographic")
        .arg("-hda")
        .arg(&vm_image_path)
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
        .expect("ERROR:: failed to exec VM creation command");

    if status.success() {
        println!("\nLOG:: AutoVirt VM creation success 👍");
    } else {
        eprintln!(
            "ERROR:: Something went wrong or something failed to do something with
            \nthe VM\nAUTOVIRT_DEBUG=1 and re-run for more info"
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
