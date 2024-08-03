use clap::Parser;
use clap::Subcommand;
use std::process::Command;


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

    /// Lists details about certain things (`all` VMs, `nets` all VM networks)
    
    List {

        #[arg(help = "The name of the VM to show details about")]
        name: String,

        #[arg(help = "The virtual network to show details about", default_value = "")]
        path: String,

    },
}


// /// CLI APp for VM Automation (qemu)
// ///
// #[derive(Parser)]
// enum Commands {
//     /// The name of the virtual machine to make (qemu)
//     vm_name: String,
//     /// Virtual machine type (GUI, CLI)
//     #[arg(long)]
//     vm_type: String,
//     /// VM Operating System (ubuntu_2204, rocky8)
//     #[arg(long)]
//     vm_os: String
// }

fn main() {

    // running some random comamnd for testing
    println!("Hello there");

    let cli_arguments = Cli::parse();

    match &cli_arguments.command {
        VMCommands::List { name, path } => {
            println!("hello there");
            _ = name;
            _ = path;
            let mut test_command = Command::new("ls");
            let something = test_command.status().expect("rip it didnt work");
            println!();
            _ = something;
        }

    }


}
