mod minas;

// use minas::{ generate_key::generate, pda::generate_pda };
use minas::*;

fn main() {
    generate_key::generate();
    pda::generate_pda();
}