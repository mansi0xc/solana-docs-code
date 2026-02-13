use solana_sdk::signer::{keypair::Keypair, Signer};

#[tokio::main]
async fn main() {
    let keypair = Keypair::new();
    print!("Public Key : {}", keypair.pubkey());
    print!("Private Key: {:?}", keypair.to_bytes());
}

/*
kit version:
import {generateKeyPair} from "@solana/kit";

constant keypairSigner = await generateKeyPair();
console.log(keypairSigner);
*/