use clap::Parser;
use std::process::Command;

/// CLI APp for VM Automation (qemu)
///
#[derive(Parser)]
#[command(name = "AutoVirt VM Automation CLI")]
#[command(author = "Fluffy Snowman <fluffy@fluffysnowman.dev>")]
#[command(version = "0.0.1")]
#[command(about = "VM Automation Since I'm a Lazy fat fuck", long_about = None)]

struct Cli {

    /// The name of the virtual machine to make (qemu)
    vm_name: String,
}

fn main() {
    let args = Cli::parse();

    let output = Command::new("virsh")
        .arg("dominfo")
        .arg(&args.vm_name)
        .output()
        .expect("Failed to excetue vm dom info");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("ERROR: {}", String::from_utf8_lossy(&output.stderr));
    }
}
