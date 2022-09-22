pub const BLOCK_TIME_SECS: u64 = 10;
const GENESIS_ACCOUNT_ID: u32 = u32::MAX;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
}

pub struct Block {
    pub index: usize,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone)]
pub struct Transaction {
    pub sender: u32,
    pub recipient: u32,
    pub amount: i32
}

impl Blockchain {

    pub fn new() -> Blockchain {
        let mut first_block = Block::new();
        first_block.transactions = vec![];
        Blockchain { chain: vec![first_block], current_transactions: Vec::new() }
	}

	/// Adds new block to the chain
	pub fn new_block(&mut self) {
        let block = Block {
            index: self.last_block_index() + 1,
            transactions: self.current_transactions.clone(),
        };

        self.current_transactions = Vec::new();
        self.chain.push(block);
		println!("new block added (transactions count : {})", &self.chain[self.last_block_index()].transactions.len());
	}

	/// Adds a account
	pub fn new_account(&mut self, account: u32, amount: i32) {
		let new_transaction = Transaction::new(GENESIS_ACCOUNT_ID, account, amount);
        self.current_transactions.push(new_transaction);
	}
	
	/// Adds a transaction
	pub fn new_transaction(&mut self, sender: u32, recipient: u32, amount: i32) {
		let new_transaction = Transaction::new(sender, recipient, amount);
        self.current_transactions.push(new_transaction);
	}

	/// Returns account balance
	pub fn get_balance(&self, account: u32) -> i32 {
		let mut balance = 0;

		for block in &self.chain {
            for transaction in &block.transactions {
				if transaction.sender == account {
					balance -= transaction.amount;
				}
				if transaction.recipient == account {
					balance += transaction.amount;
				}
            }
        }

		balance
	}

	fn last_block_index(&self) -> usize {
        self.chain.len()  - 1
    }
}

impl Block {
    fn new() -> Block {
        Block { index: 0, transactions: Vec::new()}
    }
}

impl Transaction {
    fn new(sender: u32, recipient: u32, amount: i32) -> Transaction {
        Transaction { sender, recipient, amount }
    }
}
