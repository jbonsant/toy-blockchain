# Toy Blockchain

## Installation

1. Clone the repo
2. Install rust and cargo https://rustup.rs/

## Running

1. Enter toy-blockchain project directory

```
cd toy-blockchain
```

2. Build the project

```
cargo build --release
```

3. Enter the release folder

```
cd target/release
```

4. Start the node server

```
./b start-node
```

## Commands

### Create account

```
./b create-account <id_of_account> <starting_balance>
```

### Transfer funds

```
./b transfer <from_account> <to_account> <ammount>
```

### Show account balance

```
./b balance <account-id>
```
