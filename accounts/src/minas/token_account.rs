use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;

pub async fn fetch_token_account() -> Result<()> {
    let connection = RpcClient::new_with_commitment("https://api.mainnet.solana.com".to_string(),CommitmentConfig::confirmed());

    let program_id = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    let account_info = connection.get_account(&program_id).await?;

    println!("{:#?}", account_info);

    Ok(())
}

/*
kit version:
import { Address, createSolanaRpc } from "@solana/kit";

const rpc = createSolanaRpc("https://api.mainnet.solana.com");

const programId = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" as Address;

const accountInfo = await rpc
  .getAccountInfo(programId, { encoding: "base64" })
  .send();
console.log(accountInfo);
 */