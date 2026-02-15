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
    match create_token::create_and_fetch().await {
        Ok(_) => println!("Token creation and fetching successful"),
        Err(e) => eprintln!("Error encountered during token creation: {}", e),
    }

    match funding::fund_wallet().await {
        Ok(_) => println!("Wallet funded successfully!"),
        Err(e) => eprintln!("Error encountered during wallet funding: {}", e),
    }

    match sysvar::fetch_sys_data().await {
        Ok(_) => println!("Sysvar data fetched successfully!"),
        Err(e) => eprintln!("Error encountered sysvar data fetching: {}", e),
    }
}