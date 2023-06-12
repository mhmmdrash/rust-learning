mod blockchain;

use std::io::Write;
use std::process;
use std::io::stdin;
use std::io::stdout;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();
    loop {
        println!("Enter miner address");
        stdout().flush();
        stdin().read_line(&mut miner_address);
        println!("Enter difficulty");
        stdout().flush();
        stdin().read_line(&mut difficulty);
        let diff = difficulty
            .trim()
            .parse::<u32>()
            .expect("must enter an integer");
        println!("Generating Genesis block");
        let mut chain = blockchain::Chain::new(miner_address, diff);

        loop {
            println!("Welcome to the Arcadia mainnet");
            println!("Choose one of the options below to interact with the network");
            println!("1. Make a transaction");
            println!("2. Mine a block");
            println!("3. Change the difficulty");
            println!("4. Change the reward");
            println!("0. Exit");
            stdout().flush();
            println!("Enter option");
            choice.clear();
            stdin().read_line(&mut choice);
            
            match choice.trim().parse().unwrap() {
                0 => {
                    println!("Exiting");
                    process::exit(0);
                }
                1 => {
                    let mut sender = String::new();
                    let mut receiver = String::new();
                    let mut amount = String::new();
                    println!("Enter sender name");
                    stdout().flush();
                    stdin().read_line(&mut sender);
                    println!("Enter receiver name");
                    stdout().flush();
                    stdin().read_line(&mut receiver);
                    println!("Enter amount");
                    stdout().flush();
                    stdin().read_line(&mut amount);
                    let am = amount 
                        .trim()
                        .parse::<f32>()
                        .expect("Must enter an integer for amount");

                    println!("Creating tx from {} to {} for {} units", sender, receiver, am);
                    let res = chain.new_tx(sender, receiver, am);
                    match res {
                        true => println!("transaction added"),
                        false => println!("transaction failed"),
                    }
                }
                2 => {
                    println!("Generating block...");
                    let res = chain.mine_block();
                    match res {
                        true => println!("Block added"),
                        false => println!("Block failed"),
                    }
                }
                3 => {
                    let mut difficulty = String::new();
                    println!("Enter new difficulty");
                    stdout().flush();
                    stdin().read_line(&mut difficulty);
                    let diff = difficulty
                        .trim()
                        .parse::<u32>()
                        .expect("Difficulty must be an integer value");
                    
                    let res = chain.update_difficulty(diff);
                    match res {
                        true => println!("Successfully updated difficulty"),
                        false => println!("Difficulty updation failed"),
                    }
                }
                4 => {
                    let mut reward = String::new();
                    println!("Enter new reward");
                    stdout().flush();
                    stdin().read_line(&mut reward);
                    let rew = reward
                        .trim()
                        .parse::<f32>()
                        .expect("Difficulty must be an integer value");
                    
                    let res = chain.update_reward(rew);
                    match res {
                        true => println!("Successfully updated reward"),
                        false => println!("Reward updation failed"),
                    }
                }
                _ => println!("Invalid option please retry"),
            }
        }
    }
}