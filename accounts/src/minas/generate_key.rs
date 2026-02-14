use solana_sdk::signer::{keypair::Keypair, Signer};

pub fn generate() {
    let keypair = Keypair::new();
    println!("Public Key : {}", keypair.pubkey());
    println!("Private Key: {:?}", keypair.to_bytes());
}

/*
tokio version:
use solana_sdk::signer::{keypair::Keypair, Signer};

#[tokio::main]
async fn main() {
    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());
}
*/

/*
kit version:
import {generateKeyPair} from "@solana/kit";

constant keypairSigner = await generateKeyPair();
console.log(keypairSigner);
*/