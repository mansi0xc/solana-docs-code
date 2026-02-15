use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    signer::{keypair::Keypair, Signer},
};

pub async fn fund_wallet() -> Result<()> {
    // Generate a new keypair
    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());

    // Create a connection to solana cluster
    let connection = RpcClient::new_with_commitment("http://localhost:8899".to_string(), CommitmentConfig::confirmed());

    // Funding an address with SOL automatically creates an account
    let signature = connection.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL).await?;

    loop {
        let confirmed = connection.confirm_transaction(&signature).await?;
        if confirmed {
            break;
        }
    }

    let account_info = connection.get_account(&keypair.pubkey()).await?;
    println!("{:#?}", account_info);

    Ok(())
}

/*
kit version:
import {
  airdropFactory,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  generateKeyPairSigner,
  lamports
} from "@solana/kit";

// Create a connection to Solana cluster
const rpc = createSolanaRpc("http://localhost:8899");
const rpcSubscriptions = createSolanaRpcSubscriptions("ws://localhost:8900");

// Generate a new keypair
const keypair = await generateKeyPairSigner();
console.log(`Public Key: ${keypair.address}`);

// Funding an address with SOL automatically creates an account
const signature = await airdropFactory({ rpc, rpcSubscriptions })({
  recipientAddress: keypair.address,
  lamports: lamports(1_000_000_000n),
  commitment: "confirmed"
});

const accountInfo = await rpc.getAccountInfo(keypair.address).send();
console.log(accountInfo);
 */
