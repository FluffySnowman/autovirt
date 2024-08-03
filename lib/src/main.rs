// Rust imports

use clap::Parser;
use clap::Subcommand;
// use std::process::Command;

// project imports
mod info;
mod scripts;

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
    /// Gets info & details about VMs and networks (qemu).
    Info {
        #[arg(
            help = "The name of the VM to show details about",
            default_value = "none"
        )]
        name: String,
    },
    /// Lists general VM-related items.
    List {
        #[arg(help = "The item to list")]
        item: String,
    },
    /// Create a new VM based on a given distro, user/pass and name.
    Create {
        /// The name of the new virtual machine.
        #[arg(help = "Name of the VM to be created", default_value = "basicvm")]
        name: String,

        /// The distribution (linux distro) of the VM to create.
        #[arg(
            help = "Distro of the VM to create (see options with \n`autovirt show available`)",
            default_value = "ubuntu2204"
        )]
        dist: String,

        /// The size of the new virtual machine (1G, 2G ...)
        #[arg(help = "The size of the new VM (1G, 2G ...)", default_value = "1G")]
        size: String,

        /// The suername for the VM (non-root)
        #[arg(help = "The username for the VM", default_value = "fluffy")]
        user: String,

        /// The password for the VM (non-root)
        #[arg(help = "The Password for the VM", default_value = "123456")]
        pass: String,
    },
}

fn main() {
    let cli_arguments = Cli::parse();

    match &cli_arguments.command {
        VMCommands::Info { name } => {
            info::get_vm_info(name);
        }
        VMCommands::List { item } => {
            info::show_all_vms();
            _ = item;
        }
        VMCommands::Create {
            name,
            dist,
            size,
            user,
            pass,
        } => {
            scripts::create_new_vm(name, dist, size, user, pass);
        }
    }
}
