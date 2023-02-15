use dotenv::dotenv;
use anyhow;
use gethp2p::conn_p2p;
//Main function
pub fn main()  -> anyhow::Result<()>{
    dotenv().ok();
    let node_id=std::env::var("NODE_ID")?;
    conn_p2p(node_id)?;
    Ok(())
}

