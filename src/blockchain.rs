const GENESIS_ACCOUNT_ID: u32 = u32::MAX;

#[derive(Clone)]
pub struct Transaction {
    pub sender: u32,
    pub recipient: u32,
    pub amount: i32
}

pub struct Block {
    pub index: usize,
    pub transactions: Vec<Transaction>,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
}

impl Transaction {
    fn new(sender: u32, recipient: u32, amount: i32) -> Transaction {
        Transaction { sender, recipient, amount }
    }
}

impl Block {
    fn new() -> Block {
        Block { index: 0, transactions: Vec::new()}
    }
}

impl Blockchain {

    pub fn new() -> Blockchain {
        let mut first_block = Block::new();
        first_block.transactions = vec![];
        Blockchain { chain: vec![first_block], current_transactions: Vec::new() }
	}

	pub fn new_block(&mut self) {
		println!("blockchain : new_block");

        let block = Block {
            index: self.last_block_index() + 1,
            transactions: self.current_transactions.clone(),
        };

        self.current_transactions = Vec::new();
        self.chain.push(block);
	}

	pub fn new_account(&mut self, account: u32, amount: i32) {
		let new_transaction = Transaction::new(GENESIS_ACCOUNT_ID, account, amount);
        self.current_transactions.push(new_transaction);
	}
	
	pub fn new_transaction(&mut self, sender: u32, recipient: u32, amount: i32) {
		let new_transaction = Transaction::new(sender, recipient, amount);
        self.current_transactions.push(new_transaction);
	}

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
