mod cli;
mod error;
mod modules;
mod register;
mod utils;

use register::register;
use utils::logger::Log;

#[tokio::main]
async fn main() {
    match register().await {
        Ok(_) => (),
        Err(e) => Log::default().error(e),
    }
}
