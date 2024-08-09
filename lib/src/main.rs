// Rust imports

use clap::Parser;
use clap::Subcommand;
use tokio;
use tokio::task;
use tokio::runtime::Runtime;
// use std::process::Command;

// project imports
mod info;
mod scripts;
mod download;
mod imds;

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
    /// Create a new VM based on a given distro, user/pass and name
    /// (cloud-init/qemu)
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
        #[arg(help = "The disk size of the new VM in GB: 10,25,30,etc.)", default_value = "10")]
        size: String,

        /// The suername for the VM (non-root)
        #[arg(help = "The username for the VM", default_value = "fluffy")]
        user: String,

        /// The password for the VM (non-root)
        #[arg(help = "The Password for the VM", default_value = "123456")]
        pass: String,

        /// The amount of memory in MB (Example: 512 or 1024)
        #[arg(help = "The amount of memory for the VM", default_value = "512")]
        mem: String,
    },
    /// Downloads a cloud-init compatible image for the specified distro.
    Download {
        /// The distro (linux distribution) of the image to download
        #[arg(
            help = "The VM Image (cloud init/qemu) to download.\nSee `autovirt show available` for a full list\nof available images to download.",
        )]
        dist: String,
    }
}

#[tokio::main]
async fn main() {
    let cli_arguments = Cli::parse();

    // Setting imds http server params (used for cloud-init/vm config files).
    // This includes user data, metadata and other shit.
    let _imds_listen_host = "0.0.0.0";
    let _imds_listen_port = "8000";
    let _imds_data_dir = "./lib/src/conf/";

    // File server is run in the create command section.

    match &cli_arguments.command {
        VMCommands::Info { name } => {
            info::get_vm_info(name);
        }
        VMCommands::List { item } => {
            // in fo::show_all_vms();
            download::available_images();
            _ = item;
        }
        VMCommands::Create {
            name,
            dist,
            size,
            user,
            pass,
            mem,
        } => {
            // File server is currently run with hardcoded values since the
            // compiler keeps yapping.
            //
            // It is run async or in its own thread so that it doesn't block the
            // vm startup and creation etc.

            tokio::spawn(async move {
                imds::start_idms_server();
            });

            // imds::run_file_server(imds_addr, imds_data_dir).await;
            scripts::create_new_vm(name, dist, size, user, pass, mem);
        }
        VMCommands::Download { dist } =>  {
            println!("Downloading OS Image for {}", dist);
            download::download_vm_image();
            println!("OS downloaded -> {}", dist);
        }
    }
}
