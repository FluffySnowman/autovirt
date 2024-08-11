// Rust imports

use clap::Parser;
use clap::Subcommand;
use tokio;
// use tokio::task;
// use tokio::runtime::Runtime;
// use std::process::Command;

// project imports
mod info;
mod scripts;
mod download;
mod imds;
mod filesystem;

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
    /// Populates config files with data required for autovirt to work.
    Init { },
    /// Creates config files and data directory(s) for autovirt.
    Install { },
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
        #[arg(short, long, help = "Name of the VM to be created", default_value = "basicvm")]
        name: String,

        /// The distribution (linux distro) of the VM to create.
        #[arg(
            short,
            long,
            help = "Distro of the VM to create (see options with \n`autovirt show available`)",
            default_value = "ubuntu2204"
        )]
        dist: String,

        /// The size of the new virtual machine (1G, 2G ...)
        #[arg(short, long, help = "The disk size of the new VM in GB: 10,25,30,etc.)", default_value = "10")]
        size: String,

        /// The suername for the VM (non-root)
        #[arg(short, long, help = "The username for the VM", default_value = "fluffy")]
        user: String,

        /// The password for the VM (non-root)
        #[arg(short, long, help = "The Password for the VM", default_value = "123456")]
        pass: String,

        /// The amount of memory in MB (Example: 512 or 1024)
        #[arg(short, long, help = "The amount of memory for the VM", default_value = "512")]
        mem: String,

        /// The number of vCPU' s for the VM
        #[arg(short, long, help = "The amount of vCPU's for the vm", default_value = "1")]
        cpus: String,

        // /// The path to an already existing image (.img cloud init file)
        // #[arg(short, long, help = "Path to existing cloud init .img file", default_value = "1")]
        // path: String,
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

    // Init the distro image list
    download::init_available_images();

    // File server is run in the create command section.

    match &cli_arguments.command {
        VMCommands::Install {  } => {
            println!("WARNING:: ONLY RUN THIS COMMAND ONCE.");
            println!("WARNING:: IF YOU HAVE ALREADY RUN THIS COMMAND THEN IT WILL OVERWRITE");
            println!("WARNING:: THE EXISTING DATA DIRECTORY AND WILL CAUSE DATA LOSS.");
            println!("Proceed with installation? [yes/No] ");
            let mut proceed_prompt = String::new();
            if { std::io::stdin().read_line(&mut proceed_prompt).unwrap(); proceed_prompt.trim().eq_ignore_ascii_case("yes") } {
                println!("INFO:: installing autovirt...");
                println!("INFO:: Creating data directories for autovirt...");
                match filesystem::create_autovirt_data_dir() {
                    Ok(()) => {
                        println!("SUCCESS: Autovirt data directory created successfully");
                    },
                    Err(e) => eprintln!("ERROR: Failed to create autovirt data directory -> {}", e),
                }

            } else {
               println!("ABORTED AUTOVIRT INSTALLATION");
            }

        }
        VMCommands::Init {  } => {

            // let test_img: String = "ubuntu2204".to_string();

            // Testing shit
            // let _ = filesystem::insert_autovirt_config_data();

            println!("INFO:: INitialising autovirt config data");
        },
        VMCommands::Info { name } => {
            info::get_vm_info(name);
        }
        VMCommands::List { item } => {
            // in fo::show_all_vms();
            // download::available_images();
            println!("listing available images");
            _ = item;
        }
        VMCommands::Create {
            name,
            dist,
            size,
            user,
            pass,
            mem,
            cpus,
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
            scripts::create_new_vm(name, dist, size, user, pass, mem, cpus);
            // exit everythnig
            std::process::exit(0);
        }
        VMCommands::Download { dist } =>  {
            _ = dist;
            // let _ = download::download_vm_image(&dist.to_string());
        }
    }
}
