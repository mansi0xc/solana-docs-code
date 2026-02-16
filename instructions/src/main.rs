mod wince;
use wince::*;

#[tokio::main]
async fn main() {
    if let Err(e) = transfer_sol::transfer_from().await {
        eprintln!("transfer failed: {:#?}", e);
    }
}
