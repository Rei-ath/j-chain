mod handler;
mod models;
mod utils;
mod mote;
mod test;
mod vs;
mod bc;
use bc::block_chain::Blockchain;
use handler::handle_connection;
use tokio::net::TcpListener;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

// #[derive(Debug)]
lazy_static! {
    #[derive(Debug)]
    pub static ref BLOCKCHAIN: Arc<Mutex<Blockchain>> = Arc::new(Mutex::new(Blockchain::new(3)));
}

#[tokio::main]
async fn main() {
    // let mut block_chain = Blockchain::new(3);
    let addr = "127.0.0.1:8080";
    test::test_2();
    // let bc = 
    // BLOCKCHAIN.lock();
    // println!("{:?}",BLOCKCHAIN);
    // let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    // println!("Server running on ws://{}", addr);

    // // Accept incoming connections
    // while let Ok((stream, _)) = listener.accept().await {
    //     // Spawn a new task to handle each incoming connection
    //     tokio::spawn(handle_connection(stream));
    // }
}
