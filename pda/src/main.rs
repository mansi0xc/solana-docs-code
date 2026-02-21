mod parse;

use parse::*;

fn main() {
    all_bumps::get_all_bumps();
    address_seed::get_pda_from_address_seed();
    multiple_seeds::pda_from_multiple_seeds();
}
