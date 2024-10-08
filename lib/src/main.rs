// Rust imports

use clap::Parser;
use clap::Subcommand;
use tokio;
// use tokio::task;
// use tokio::runtime::Runtime;
// use std::process::Command;

// project imports
mod info;
mod create;
mod run;
mod download;
mod imds;
mod filesystem;
mod vmutils;
mod initdata;

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
    /// Creates config files and data directory(s) for autovirt.
    Install { },
    /// Populates config files with data required for autovirt to work.
    Init { },
    /// Gets info & details about VMs by name.
    Info {
        #[arg(
            help = "The name of the VM to show details about",
            required=true,
            default_value = "none"
        )]
        name: String,

        #[arg(short, long, help = "Print raw json")]
        raw: bool,

    },
    /// Lists all currently installed VMs with some metadata.
    List { },
    /// Create a new VM based on a given distro, user/pass and name
    /// (cloud-init/qemu)
    Create {
        /// The name of the new virtual machine.
        #[arg(short, long, required=true, help = "Name of the VM to be created", default_value = "basicvm")]
        name: String,

        /// The distribution (linux distro) of the VM to create.
        #[arg(
            short,
            long,
            required=true,
            help = "Distro of the VM to create (see options with \n`autovirt show available`)",
            default_value = "ubuntu2204"
        )]
        dist: String,

        /// The size of the new virtual machine (1G, 2G ...)
        #[arg(short, long, required=true, help = "The disk size of the new VM in GB: 10,25,30,etc.)", default_value = "10")]
        size: String,

        /// The suername for the VM (non-root)
        #[arg(short, long, required=true, help = "The username for the VM", default_value = "fluffy")]
        user: String,

        /// The password for the VM (non-root)
        #[arg(short, long, required=true, help = "The Password for the VM", default_value = "123456")]
        pass: String,

        /// The amount of memory in MB (Example: 512 or 1024)
        #[arg(short, long, required=true, help = "The amount of memory for the VM", default_value = "512")]
        mem: String,

        /// The number of vCPU' s for the VM
        #[arg(short, long, required=true, help = "The amount of vCPU's for the vm", default_value = "1")]
        cpus: String,

        /// The path to an ssh key to add to the user
        #[arg(short, long, required=true, help = "Path to an ssh key to add to the user", default_value = "none")]
        key: String,

        /// String for port forwarding arguments
        #[arg(short, long, help = "Port forward args (i.e. -> 'hostfwd=tcp::2244-:22' )", default_value = "")]
        ports: String,

        // /// The path to an already existing image (.img cloud init file)
        // #[arg(short, long, help = "Path to existing cloud init .img file", default_value = "1")]
        // path: String,
    },
    /// Runs the specified virtual machine (identified by name)
    Run  {
        /// The name of the virtual machine to run
        #[arg(required=true, help = "Name of the VM to run")]
        name: String,

        /// String for port forwarding arguments
        #[arg(short, long, help = "Port forward args (i.e. -> 'hostfwd=tcp::2244-:22' )", default_value = "")]
        ports: String,
    },
    /// Downloads a cloud-init compatible image for the specified distro.
    Download {
        /// The distro (linux distribution) of the image to download
        #[arg(
            help = "The VM Image (cloud init/qemu) to download.\nSee `autovirt show available` for a full list\nof available images to download.",
            required=true,
        )]
        dist: String,
    },
    /// Show various things such as available images to download
    Show {
        /// Show available images to download
        #[arg(short, long, help = "Show available images to download")]
        available: bool,
    },
    /// Resize the VM disk, memory, cpu etc.
    Resize {
        /// The name of the VM to resize
        #[arg(short, required=true, long, help = "Name of the VM to resize")]
        name: String,

        /// Relative size to increase the disk by (ONLY POSITIVE VALUES)
        #[arg(short, long, required=true,  help = "The relative size to increase the disk in GB.\nSuch as 5, 10, 20 etc.", default_value = "0")]
        disk: String,

        /// New memory size in MB
        #[arg(short, long, required=true,  help = "New memory size in MB (512, 1024, 2048 etc.")]
        memory: String,

        /// New amount of CPUs
        #[arg(short, long, required=true,  help = "The new amount of CPUs for the VM (1, 2, 4 etc.")]
        cpus: String,
    },
    /// Clone a specified VM by name to a new VM with a new name.
    Clone {
        /// The name of the VM to clone
        #[arg(required=true, help = "Name of the VM to clone")]
        name: String,

        /// The new name of the VM to clone to
        #[arg(required=true, help = "Name of the new VM")]
        new_name: String,
    },
    /// Deletes specified VM (by name) along with associated files & relevant
    /// configs.
    Delete {
        #[arg(required=true, help = "Name of the VM to delete")]
        name: String,
    },
    /// Gets the checksum of a specified file/image.
    Checksum {
        /// The file to get the checksum of
        #[arg(help = "The file to get the checksum of")]
        file: String,
    },
    /// Checks autovirt config file for errors & downloaded vm checksums
    Health { }
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
            println!("WARNING:: ONLY RUN THIS COMMAND ONCE.");
            println!("WARNING:: IF YOU HAVE ALREADY RUN THIS COMMAND THEN IT WILL");
            println!("WARNING:: OVERWRITE THE EXISTING CONFIG FILE autovirt.json.\n");
            println!("WARNING:: The autovirt.json file contains important data for autovirt to work");
            println!("WARNING:: such as all the metadata for the installed VM's, the available images");
            println!("WARNING:: and other important data.\n");
            println!("Proceed with initialisation? [yes/No] ");
            let mut proceed_prompt = String::new();
            if { std::io::stdin().read_line(&mut proceed_prompt).unwrap(); proceed_prompt.trim().eq_ignore_ascii_case("yes") } {
                println!("INFO:: Initialising autovirt...");
                println!("INFO:: Creating config file for autovirt...");
                match filesystem::insert_autovirt_config_data() {
                    Ok(()) => {
                        println!("SUCCESS: Autovirt config file created successfully");
                    },
                    Err(e) => eprintln!("ERROR: Failed to create autovirt config file -> {}", e),
                }

            } else {
               println!("ABORTED AUTOVIRT INITIALISATION");
            }
        },
        VMCommands::Info { name, raw } => {
            vmutils::get_vm_info_by_name(name, *raw);
        }
        VMCommands::List { } => {
            println!("\n------ All Installed VMs ------\n");
            vmutils::list_vms();
        }
        VMCommands::Create {
            name,
            dist,
            size,
            user,
            pass,
            mem,
            cpus,
            key,
            ports,
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
            create::create_new_vm(name, dist, size, user, pass, mem, cpus, key, ports);
            // exit everythnig
            std::process::exit(0);
        }
        VMCommands::Run { name, ports } => {
            run::run_vm(name, ports);
        }
        VMCommands::Download { dist } =>  {
            let distro_link = filesystem::get_value_from_autovirt_json(&format!("images.{}.link", dist));
            match distro_link {
                Some(link) => println!("Fetched download link for {}: -> {}", dist, link.as_str().unwrap_or("Invalid link")),
                None => println!("ERROR: Could not find a download link for istro -> {}", dist),
            }

            let _ = download::download_vm_image(dist);

            // let _ = download::download_vm_image(&dist.to_string());
        },
        VMCommands::Show { available  } => {
            _ = available; // this is meant to be unused
            vmutils::show_available_images();
        }
        VMCommands::Resize { name, disk, memory, cpus } => {
            vmutils::resize_vm(name, disk, memory, cpus);
        },
        VMCommands::Clone { name, new_name } => {
            vmutils::clone_vm(name, new_name);
        },
        VMCommands::Delete { name } => {
            println!("Deleting VM (name): {}", name);
            vmutils::delete_vm(name);
            print!("VM Deleted (or not idk bruh)");
        },
        VMCommands::Checksum { file } => {
            vmutils::get_image_checksum(file);
        }
        VMCommands::Health {  } => {
            println!("Checking autovirt data and config file for errors and checksums for vms...");
        }
    }
}

