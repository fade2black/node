use std::sync::Arc;

use etcd_client::Error;
use node::{get_args, State};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let args = get_args();

    let state = Arc::new(RwLock::new(State::new()));
    node::run(&args, state).await?;

    Ok(())
}
