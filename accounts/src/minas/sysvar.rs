use anyhow::Result;
use bincode::deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    sysvar::{self, clock::Clock},
};

pub async fn fetch_sys_data() -> Result<()> {
    let connection = RpcClient::new_with_commitment("https://api.mainnet.solana.com".to_string(), CommitmentConfig::confirmed());

    let account = connection.get_account(&sysvar::clock::ID).await?;
    // Deserialize the account data
    let clock: Clock = deserialize(&account.data)?;

    println!("{:#?}", account);
    println!("{:#?}", clock);

    Ok(())
}

/*
kit version:
import { createSolanaRpc } from "@solana/kit";
import { fetchSysvarClock, SYSVAR_CLOCK_ADDRESS } from "@solana/sysvars";

const rpc = createSolanaRpc("https://api.mainnet.solana.com");

const accountInfo = await rpc
  .getAccountInfo(SYSVAR_CLOCK_ADDRESS, { encoding: "base64" })
  .send();
console.log(accountInfo);

// Automatically fetch and deserialize the account data
const clock = await fetchSysvarClock(rpc);
console.log(clock);
 */