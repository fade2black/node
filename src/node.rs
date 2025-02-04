use std::{sync::Arc, time::Duration};

use etcd_client::{Client, Error};
use tokio::{sync::RwLock, time};
use tracing::{info, error};
use crate::args::Args;

const ELECTION_NAME: &str = "leader_election";
const TTL: i64 = 10;

pub struct State {
    is_leader: bool,
}

impl State {
    pub fn new() -> Self {
        Self { is_leader: false}  
    }
}

pub async fn run(args: &Args, state: Arc<RwLock<State>>) -> Result<(), Error> {

    let res = tokio::try_join!(
        participate_in_election(args, state.clone()),
        observe_election(args, state.clone()),
    );

    if let Err(err) = res {
        error!("{err}");
    }

    info!("Terminating.");

    Ok(())
}

async fn participate_in_election(args: &Args, _state: Arc<RwLock<State>>) -> Result<(), Error> {    
    let mut client = connect(args).await?;

    loop {
        let resp = client.lease_grant(TTL, None).await?;
        let lease_id = resp.id();
        
        info!("Starting a new campaign.");
        let resp = client.campaign(ELECTION_NAME, args.node.clone(), lease_id).await?;
        let leader_key = resp.leader().unwrap();
        info!("🥳 I am the leader ({})", args.node);

        if let Ok((mut keeper, _)) = client.lease_keep_alive(lease_id).await {
            loop {
                info!("⏰ Keeping alive the lease {}...", leader_key.key_str()?);
                keeper.keep_alive().await?;
                time::sleep(Duration::from_secs(7)).await;
            }
        } else {
            error!("Failed to keep lease alive. Re-campaigning.");
        }
    }
}

async fn observe_election(args: &Args, state: Arc<RwLock<State>>) -> Result<(), Error> {
    let mut client = connect(args).await?;

    let mut msg = client.observe(ELECTION_NAME).await?;
    loop {
        if let Some(resp) = msg.message().await? {
            let kv = resp.kv().ok_or(Error::WatchError("Unable to retrieve key/value".into()))?;
            let key = kv.key_str()?;
            let val = kv.value_str()?; 
            
            let mut st = state.write().await;
            (*st).is_leader = val == args.node;
            info!("🟢 Current leader is {val} with key {key}, node.is_leader={}", (*st).is_leader);
        }
    }
}

async fn connect(args: &Args) -> Result<Client, Error> {
    let server = format!("{}:{}", args.host, args.port);

    info!("Connecting to etcd server.");
    let client = Client::connect([server], None).await?; 
    info!("Connection to etcd server established.");

    Ok(client)
}