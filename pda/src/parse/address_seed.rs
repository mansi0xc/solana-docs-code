use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn get_pda_from_address_seed() -> anyhow::Result<()> {
    let program_id = Pubkey::from_str("11111111111111111111111111111111")?;
    let optional_address_seed = Pubkey::from_str("B9Lf9z5BfNPT4d5KMeaBFx8x1G4CULZYR1jA2kmxRDka")?;
    let seeds: &[&[u8]] = &[optional_address_seed.as_ref()];
    let (pda, bump) = Pubkey::find_program_address(seeds, &program_id);

    println!("PDA: {}", pda);
    println!("Bump: {}", bump);
    Ok(())
}