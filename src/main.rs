extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

mod transaction;
mod wallet;
mod market;

use dotenv::dotenv;

// Launches the program
fn main() {
    dotenv().ok();

    start();
    run();
    stop();
}

// Displays the start status for the program
fn start() {
    println!("\nCrypto project starting...\n")
}

// Bootstraps the main logic
fn run() {
    transaction::show();
    wallet::show();
}

// Displays the stop status for the program
fn stop() {
    println!("Crypto project terminated, bye")
}
