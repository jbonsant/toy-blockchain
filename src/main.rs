use clap::{Parser, Subcommand};
mod blockchain;
use crate::blockchain::Blockchain;
use tokio::net::UdpSocket;
use tokio::time::{Duration};
use std::io;
use std::error::Error;
use bytes::{Bytes, BytesMut, BufMut};

const NODE_ADDR: &str = "0.0.0.0:8080";
const CLIENT_ADDR: &str = "0.0.0.0:8081";
const BLOCK_TIME_SECS: u64 = 2;

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
		starting_balance: i32,
	},
	Transfert {
		from_account: u32, 
		to_account: u32,
		ammount: i32,
	},
	Balance {
		account: u32,
	},
}

fn args_to_bytes(args: std::env::Args) -> Vec<u8> {

	let mut bytes = BytesMut::new();
	
	for arg in args {
		bytes.put(format!("{}{}", arg, '\0').as_bytes());
	}

	bytes.to_vec()
}

fn bytes_to_args(bytes: Bytes) -> Vec<String> {

	let mut args: Vec<String> = Vec::new();

	for item in bytes.split(|c| c == &(0)) {
		if item.len() > 0 {
			args.push(String::from_utf8_lossy(item).to_string());
		}
	}

	args
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

	let mut rcv_data = [0; 1024];

	let cli = Cli::parse();

	match cli.command {

		Commands::StartNode {} => {
			println!("Starting Node ...");

			let mut chain = Blockchain::new();
			let mut new_block_interval = tokio::time::interval(Duration::from_secs(BLOCK_TIME_SECS));
			let mut next_block_messages: Vec<String> = Vec::new();
			
			match UdpSocket::bind(NODE_ADDR).await {
				Err(e) => {
					if e.kind() == io::ErrorKind::AddrInUse {
						println!("Node already started !")
					}
					return Err(e.into());
				}
				Ok(socket) => {
					loop {
						tokio::select! {
							_ = new_block_interval.tick() => {
								chain.new_block();
						
								for msg in &next_block_messages {
									socket.send_to(msg.as_bytes(), CLIENT_ADDR).await?;
								}
							}
							Result::Ok(rcv_data_len) = socket.recv(&mut rcv_data) => {
								
								let bytes = Bytes::copy_from_slice(&rcv_data[..rcv_data_len]);
								let args = bytes_to_args(bytes);
								let cli = Cli::parse_from(args.into_iter());

								match cli.command {
									Commands::CreateAccount { id_of_account, starting_balance } => {
										chain.new_account(id_of_account, starting_balance);
										next_block_messages.push(format!("Account created"));
									},
									Commands::Transfert { from_account , to_account, ammount } => {
										chain.new_transaction(from_account, to_account, ammount);
										next_block_messages.push(format!("Transaction done"));
									},
									Commands::Balance { account } => {
										
										let ammount: i32 = chain.get_balance(account);
										let msg = format!("Balance of account '{:?}' is {:?}", account, ammount);
										socket.send_to(msg.as_bytes(), CLIENT_ADDR).await?;
									}
									_ => {}
								}	
							}
						}
					}
				}
			}
		},
		_ => {

			let socket = UdpSocket::bind(CLIENT_ADDR).await?;
			socket.connect(NODE_ADDR).await?;

			match cli.command {
				Commands::CreateAccount { id_of_account, starting_balance } => {
					println!("Creating Account : id = {:?}, balance = {:?}\n...", id_of_account, starting_balance);
				},
				Commands::Transfert { from_account , to_account, ammount } => {
					println!("Transferring funds : from = {:?}, to = {:?}, ammount = {:?}\n...", from_account, to_account, ammount);
				},
				_ => {}
			}

			let args_data = args_to_bytes(std::env::args());
			socket.send(&args_data).await?;

			let len = socket.recv(&mut rcv_data).await?;
			println!("{}", String::from_utf8_lossy(&rcv_data[..len]));
		}
	}

	Ok(())
}