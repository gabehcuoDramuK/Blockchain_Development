mod block;
mod blockchain;

use blockchain::Blockchain;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    
    // Initialize the blockchain

    let mut blockchain = Blockchain::new(4);
    
    if blockchain.is_valid() {
        println!("Initializing Blockchain..");
    } else  {
        println!("Blockchain is Invalid at Initialization");
    }
    let mut indexer = 0;

    blockchain.balances.insert("Alice".to_string(), 500);
    blockchain.balances.insert("Bob".to_string(), 500);
    blockchain.balances.insert("Charlie".to_string(), 500);

    // Infinite loop to continuously create blocks

    loop {

        // Adding some initial balances
        // Every 5 Blocks, create transfers between accounts
        // and shows the balances on the "blockchain"
        

        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Adding a new block to the blockchain with the provided data
        blockchain.add_block(data);

        if indexer % 5 == 0 {
            let accounts = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
            let sender = &accounts[rand::thread_rng().gen_range(0..accounts.len())];
            let receiver = &accounts[rand::thread_rng().gen_range(0..accounts.len())];

            if sender != receiver {
                let amount = rand::thread_rng().gen_range(1..=10);
                blockchain.transfer_balance(sender.clone(), receiver.clone(), amount);
            }
            blockchain.show_balances();
        }

        // Add a small delay to simulate mining time
        thread::sleep(Duration::from_secs(1));
        indexer += 1;
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod extra_tests;
