use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "b")]
#[clap(about = "Toy Blockchain CLI", long_about = None)]
struct Cli {
	#[clap(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	StartNode {
	},
	CreateAccount {
		id_of_account: u32, 
		starting_balance: u32,
	},
	Transfert {
		from_account: u32, 
		to_account: u32,
		ammount: u32,
	},
	Balance {
		account: u32,
	},
}

fn main() {
	let cli = Cli::parse();

	// You can check for the existence of subcommands, and if found use their
	// matches just as you would the top level app
	match &cli.command {

		Commands::StartNode { } => {
			println!("Starting Node ...")
		},
		Commands::CreateAccount { id_of_account, starting_balance } => {
			println!("Creating Account\nid: {:?}\nbalance: {:?}\n...", id_of_account, starting_balance);
		},
		Commands::Transfert { from_account , to_account, ammount } => {
			println!("Transferring funds \nfrom: {:?}\nto: {:?}\nammount: {:?}\n...", from_account, to_account, ammount);
		},
		Commands::Balance { account } => {
			let ammount: u32 = 3;
			println!("Balance of account '{:?}' is {:?}", account, ammount);
		},

	}
}