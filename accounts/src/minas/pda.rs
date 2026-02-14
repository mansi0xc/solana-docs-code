use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub fn generate_pda() {
    let program_address = pubkey!("11111111111111111111111111111111");
    let seeds = [b"helloworld".as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);
    println!("PDA: {}", pda);
    println!("Bump: {}", bump);
}

/*
kit version:
import { Address, getProgramDerivedAddress } from "@solana/kit";

const programAddress = "11111111111111111111111111111111" as Address;

const seeds = ["helloWorld"];
const [pda, bump] = await getProgramDerivedAddress({
  programAddress,
  seeds
});

console.log(`PDA: ${pda}`);
console.log(`Bump: ${bump}`);
*/