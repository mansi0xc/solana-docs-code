mod minas;

// use minas::{ generate_key::generate, pda::generate_pda };
use minas::*;

#[tokio::main]
async fn main() {
    generate_key::generate();
    pda::generate_pda();
    match token_account::fetch_token_account().await {
        Ok(_) => println!("Token account fetched successfully"),
        Err(e) => eprintln!("Error fetching token account: {}", e),
    }
}