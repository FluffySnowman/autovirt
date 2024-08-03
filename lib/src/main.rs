// Rust imports

use clap::Parser;
use clap::Subcommand;
use info::get_vm_info;
// use std::process::Command;

// project imports
mod info;

#[derive(Parser)]
#[command(name = "AutoVirt", about = "AutoVirt VM Automation CLI", long_about = None)]
#[command(author = "Fluffy Snowman <fluffy@fluffysnowman.dev>")]
#[command(version = "0.0.1")]
#[command(about = "VM Automation Since I'm a Lazy fat fuck", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: VMCommands,
}

#[derive(Subcommand)]
enum VMCommands {

    /// Gets info & details about VMs and networks (qemu)
    Info {
        #[arg(help = "The name of the VM to show details about", default_value = "none")]
        name: String,
    },
    /// Lists general VM-related items
    List {
        #[arg(help = "Lists All VMs")]
        item: String,
    }
}

fn main() {
    let cli_arguments = Cli::parse();

    match &cli_arguments.command {
        VMCommands::Info { name } => {
            info::get_vm_info(name);
        },
        VMCommands::List { item } => {
            info::show_all_vms();
            _ = item;
        }
    }
}
