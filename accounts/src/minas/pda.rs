use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub fn generate_pda() {
    let program_address = pubkey!("11111111111111111111111111111111");
    let seeds = [b"helloworld".as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);
    println!("PDA: {}", pda);
    println!("Bump: {}", bump);
}