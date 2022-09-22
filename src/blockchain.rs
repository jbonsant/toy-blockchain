
const GENESIS_ACCOUNT_ID: u32 = u32::MAX;



/// A single transaction between a sender and recipient
pub struct Transaction {
    pub sender: u32,
    pub recipient: u32,
    pub amount: u32
}

/// A block on the chain, storing a vector of transactions and proof of work
pub struct Block {
    pub index: usize,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
}

impl Transaction {
    fn new(sender: u32, recipient: u32, amount: u32) -> Transaction {
        Transaction { sender, recipient, amount }
    }
}

impl Block {
    fn new(previous_hash: String, proof: u64) -> Block {
        Block { index: 0, transactions: Vec::new(), proof: proof, previous_hash: previous_hash }
    }
}

impl Blockchain {
    /// Create a new chain with two sender accounts credited 5000 of the token
    pub fn new() -> Blockchain {
        let first_hash = "".to_string();
        let first_proof = 100;
        let mut first_block = Block::new(first_hash, first_proof);
        let genesis_transaction_1 = Transaction::new(GENESIS_ACCOUNT_ID, 1000, 5000);
        let genesis_transaction_2 = Transaction::new(GENESIS_ACCOUNT_ID, 1001, 5000);
        first_block.transactions = vec![genesis_transaction_1, genesis_transaction_2];
        Blockchain { chain: vec![first_block], current_transactions: Vec::new() }
	}

	pub fn new_block(&self) {
		println!("blockchain : new_block");
	}

	pub fn new_account(&self, account: u32, amount: u32) {

	}
	
	pub fn new_transaction(&self, sender: u32, recipient: u32, amount: u32) {

	}

	pub fn get_balance(&self, account: u32) -> u32 {
		return 77;
	}
}
