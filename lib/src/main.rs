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

fn main() {
    let cli_arguments = Cli::parse();

    match &cli_arguments.command {
        VMCommands::List { name, path } => {
            _ = name;
            println!("hello there");
            let mut test_command = Command::new("stat");
            test_command.arg(path);
            let something = test_command.status().expect("Command Failed");
            println!();
            _ = something;
        }
    }
}
