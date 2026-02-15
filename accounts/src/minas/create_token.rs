use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{
  program_pack::Pack,
  signature::{Keypair, Signer},
  system_instruction::create_account,
  transaction::Transaction,
};
use spl_token_2022_interface::{
    id as token_2022_program_id, instruction::initialize_mint, state::Mint
};

pub async fn create_and_fetch() -> Result<()> {
    // Create connection to local validator
    let client = RpcClient::new_with_commitment(String::from("http://localhost:8899"), CommitmentConfig::confirmed(),);

    let recent_blockhash = client.get_latest_blockhash().await?;

    // Generate a new keypair for the fee payer
    let fee_payer = Keypair::new();

    // Airdrop 1 SOL to fee payer
    let airdrop_signature = client.request_airdrop(&fee_payer.pubkey(), 1_000_000_000).await?;

    loop {
        let confirmed = client.confirm_transaction(&airdrop_signature).await?;
        if confirmed {
            break;
        }
    }

    // Generate keypair to use as address of mint
    let mint = Keypair::new();

    let space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(space).await?;

    // Create account instruction
    let create_account_instruction = create_account(
        &fee_payer.pubkey(),
        &mint.pubkey(),
        rent,
        space as u64,
        &token_2022_program_id()
    );

    // Initialize mint instruction
    let initialize_mint_instruction = initialize_mint(
        &token_2022_program_id(),
        &mint.pubkey(),
        &fee_payer.pubkey(),
        Some(&fee_payer.pubkey()),
        9
    )?;

    // Create transaction and add instructions
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        recent_blockhash
    );

    // Send and confirm transaction
    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    println!("Mint Address: {}", mint.pubkey());
    println!("Transaction signature: {}", transaction_signature);

    let account_info = client.get_account(&mint.pubkey()).await?;
    println!("{:#?}", account_info);

    let mint_account = Mint::unpack(&account_info.data)?;
    println!("{:#?}", mint_account);

    Ok(())
}

/*
kit version:
import {
  airdropFactory,
  appendTransactionMessageInstructions,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  generateKeyPairSigner,
  getSignatureFromTransaction,
  lamports,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners
} from "@solana/kit";
import { getCreateAccountInstruction } from "@solana-program/system";
import {
  getInitializeMintInstruction,
  getMintSize,
  TOKEN_2022_PROGRAM_ADDRESS,
  fetchMint
} from "@solana-program/token-2022";

// Create Connection, local validator in this example
const rpc = createSolanaRpc("http://localhost:8899");
const rpcSubscriptions = createSolanaRpcSubscriptions("ws://localhost:8900");

// Generate keypairs for fee payer
const feePayer = await generateKeyPairSigner();

// Fund fee payer
await airdropFactory({ rpc, rpcSubscriptions })({
  recipientAddress: feePayer.address,
  lamports: lamports(1_000_000_000n),
  commitment: "confirmed"
});

// Generate keypair to use as address of mint
const mint = await generateKeyPairSigner();

// Get default mint account size (in bytes), no extensions enabled
const space = BigInt(getMintSize());

// Get minimum balance for rent exemption
const rent = await rpc.getMinimumBalanceForRentExemption(space).send();

// Instruction to create new account for mint (token 2022 program)
// Invokes the system program
const createAccountInstruction = getCreateAccountInstruction({
  payer: feePayer,
  newAccount: mint,
  lamports: rent,
  space,
  programAddress: TOKEN_2022_PROGRAM_ADDRESS
});

// Instruction to initialize mint account data
// Invokes the token 2022 program
const initializeMintInstruction = getInitializeMintInstruction({
  mint: mint.address,
  decimals: 9,
  mintAuthority: feePayer.address
});

const instructions = [createAccountInstruction, initializeMintInstruction];

// Get latest blockhash to include in transaction
const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

// Create transaction message
const transactionMessage = pipe(
  createTransactionMessage({ version: 0 }), // Create transaction message
  (tx) => setTransactionMessageFeePayerSigner(feePayer, tx), // Set fee payer
  (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx), // Set transaction blockhash
  (tx) => appendTransactionMessageInstructions(instructions, tx) // Append instructions
);

// Sign transaction message with required signers (fee payer and mint keypair)
const signedTransaction =
  await signTransactionMessageWithSigners(transactionMessage);

// Send and confirm transaction
await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
  signedTransaction,
  { commitment: "confirmed" }
);

// Get transaction signature
const transactionSignature = getSignatureFromTransaction(signedTransaction);

console.log("Mint Address:", mint.address);
console.log("Transaction Signature:", transactionSignature);

const accountInfo = await rpc.getAccountInfo(mint.address).send();
console.log(accountInfo);

const mintAccount = await fetchMint(rpc, mint.address);
console.log(mintAccount);
 */