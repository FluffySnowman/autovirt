//! This e File contains things related to running vms.
//!
//! Most things in here interact with the autovirt.json config file and perform
//! actions based on that.

use crate::filesystem;
use std::process::Command;
use std::thread;
use std::time;

pub fn run_vm(vm_name: &String) {
    println!("LOG:: Executing VM startup process in 3 seconds...");
    let startup_wait = time::Duration::from_secs(3);
    thread::sleep(startup_wait);

    println!("\x1b[0;32mLOG:: Starting VM...\x1b[0m");

    // ======== Getting the VM details from the autovirt config file ========
    let vm_memory_mb_json =
        filesystem::get_value_from_autovirt_json(&format!("vms.{}.memory_mb", vm_name))
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "512".to_string()); // 512 mb if shit goes down

    let vm_cpus_json = filesystem::get_value_from_autovirt_json(&format!("vms.{}.cpus", vm_name))
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "1".to_string()); // default 1 cpu if not found

    let vm_distro_json =
        filesystem::get_value_from_autovirt_json(&format!("vms.{}.distro", vm_name))
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "ubuntu2204".to_string()); // default to ubuntu2204
                                                          // but that doesn't matter
                                                          // since its just a label

    println!("-------- VM Details --------");
    println!("NAME: {}", vm_name);
    println!("DISTRO: {}", vm_distro_json);
    println!("MEMORY: {}MB", vm_memory_mb_json);
    println!("CPUS: {}", vm_cpus_json);
    println!("-----------------------------");


    // Building cmd to run the VM
    let mut run_vm_cmd = Command::new("qemu-system-x86_64");
    run_vm_cmd
        .arg("-net")
        .arg("nic")
        .arg("-net")
        .arg("user,hostfwd=tcp::2222-:22") // forwarding SSH to 2222 on host
        .arg("-machine")
        .arg("accel=kvm:tcg")
        .arg("-m")
        .arg(&vm_memory_mb_json)
        .arg("-nographic")
        .arg("-hda")
        .arg("")
        .arg("-smbios")
        .arg("type=1,serial=ds=nocloud;s=http://10.0.2.2:8000/")
        .arg("-serial")
        .arg("pty")
        .arg("-smp")
        .arg(&format!("cpus={}", vm_cpus_json));

    let status = run_vm_cmd
        .status()
        .expect("ERROR:: Failed to exec run VM command");

    if status.success() {
        println!("\nLOG:: AutoVirt run success 👍");
    } else {
        eprintln!(
            "ERROR:: Something went wrong or something failed to do something with
            \nthe VM\nAUTOVIRT_DEBUG=1 and re-run for more info"
        );
    }
}
