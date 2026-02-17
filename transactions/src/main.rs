use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    signer::keypair::Keypair,
    signature::Signer,
    transaction::Transaction,
    system_instruction::transfer
};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment("http://localhost:8899".to_string(), CommitmentConfig::confirmed());

    // Fetch the latest txn blockhash and last valid block height
    let blockhash = connection.get_latest_blockhash().await?;

    // Generate sender and recipient keypairs
    let sender = Keypair::new();
    let recipient = Keypair::new();

    // Create a transfer instruction for transferring SOL from sender to recipient
    let transfer_instruction = transfer(&sender.pubkey(), &recipient.pubkey(), LAMPORTS_PER_SOL/10);

    let mut transaction = Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
    transaction.sign(&[&sender], blockhash);

    println!("{:#?}", transaction);

    Ok(())
}

/*
retrieving details using the transaction signature and the getTransaction RPC method gives the following:
{
  "blockTime": 1745196488,
  "meta": {
    "computeUnitsConsumed": 150,
    "err": null,
    "fee": 5000,
    "innerInstructions": [],
    "loadedAddresses": {
      "readonly": [],
      "writable": []
    },
    "logMessages": [
      "Program 11111111111111111111111111111111 invoke [1]",
      "Program 11111111111111111111111111111111 success"
    ],
    "postBalances": [989995000, 10000000, 1],
    "postTokenBalances": [],
    "preBalances": [1000000000, 0, 1],
    "preTokenBalances": [],
    "rewards": [],
    "status": {
      "Ok": null
    }
  },
  "slot": 13049,
  "transaction": {
    "message": {
      "header": {
        "numReadonlySignedAccounts": 0,
        "numReadonlyUnsignedAccounts": 1,
        "numRequiredSignatures": 1
      },
      "accountKeys": [
        "8PLdpLxkuv9Nt8w3XcGXvNa663LXDjSrSNon4EK7QSjQ",
        "7GLg7bqgLBv1HVWXKgWAm6YoPf1LoWnyWGABbgk487Ma",
        "11111111111111111111111111111111"
      ],
      "recentBlockhash": "7ZCxc2SDhzV2bYgEQqdxTpweYJkpwshVSDtXuY7uPtjf",
      "instructions": [
        {
          "accounts": [0, 1],
          "data": "3Bxs4NN8M2Yn4TLb",
          "programIdIndex": 2,
          "stackHeight": null
        }
      ],
      "indexToProgramIds": {}
    },
    "signatures": [
      "3jUKrQp1UGq5ih6FTDUUt2kkqUfoG2o4kY5T1DoVHK2tXXDLdxJSXzuJGY4JPoRivgbi45U2bc7LZfMa6C4R3szX"
    ]
  },
  "version": "legacy"
}
 */