
use dotenv::dotenv;
use std::io;
use web3::transports::Http;
use web3::Web3;

// Validation function= This checks for the node id match
fn is_valid_node(fetch_node_id: &String, expected_node_id: &String) -> Result<(), std::io::Error> {
    if fetch_node_id != expected_node_id {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Rogue Node Detected",
        ));
    }
    Ok(())
}

//get connection object from web3 lib
fn get_conn(url: &str) -> Result<Web3<Http>, web3::Error> {
    let transport = Http::new(url)?;
    Ok(Web3::new(transport))
}

//This function get the p2p connection object and extracts the node information
#[tokio::main]
pub async fn conn_p2p(expected_node_id: String) -> anyhow::Result<()> {
    dotenv().ok();
    let mut choice = String::new();
    let node_endpoint = std::env::var("NODE_ENDPOINT")?;
    let connection_obj = get_conn(&node_endpoint)?;
    loop {
        let node_id = connection_obj.net().version().await?;
        is_valid_node(&node_id, &expected_node_id)?;
        let is_listening = connection_obj.net().is_listening().await?;
        if is_listening {
            println!("Is Node connected? {is_listening}");
            println!("Node ID: {node_id}");
            let client_version = connection_obj.web3().client_version().await?;
            println!("Vesion of Client in node: {client_version}");
            let peer_count = connection_obj.net().peer_count().await?;
            println!("Number of peers connected: {peer_count}");
            let chain_id = connection_obj.eth().chain_id().await?;
            println!("Current chain id: {chain_id}");
            let block_number = connection_obj.eth().block_number().await?;
            println!("Current Block Number: {block_number}" );
            println!("Enter 'x' to abort the connection or press any key to fetch new information from connected node");
            choice.clear();
            io::stdin().read_line(&mut choice).unwrap();
            if choice.trim() == "x" {
                println!("P2P handshake ended");
                break;
            } else {
                continue;
            }
        } else {
            println!("Node connection dropped")
        }
    }
    Ok(())
}

