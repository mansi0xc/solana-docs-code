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
