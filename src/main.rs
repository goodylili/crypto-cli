mod api;
mod cli;
mod utils;

use crate::cli::cli;


#[tokio::main]
async fn main() {
    cli().await;
}