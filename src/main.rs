use std::{env, time};

use bitcoincore_rpc::{
    bitcoin::{Block, Transaction},
    json,
    jsonrpc::{self},
    Auth, Client, Error, RpcApi,
};
use chrono::Duration;
use rfb_2_2024_4::utils::get_block;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

// static client: Client = Client::new("url", Auth::UserPass("user".to_owned(), "password".to_owned())).unwrap();

// TODO: Task 1
fn time_to_mine(block_height: u64) -> Result<Duration, Error> {
    // * is a deref operator which invokes the Deref trait of the type RPC_CLIENT which was created
    // when the lazy macro is expanded
    // if a value has a static lifetime then it means that value lives as long as the program lives
    let rpc_client: &Client = &*RPC_CLIENT;
    let current_block: Block = get_block(block_height, rpc_client)?;
    let prev_block: Block = get_block(block_height - 1, rpc_client)?;

    let current_block_time = current_block.header.time;
    let prev_block_time = prev_block.header.time;

    let time_to_mine_current_block = current_block_time - prev_block_time;
    Ok(Duration::new(time_to_mine_current_block.into(), 1).unwrap())
}

// TODO: Task 2
fn number_of_transactions(block_height: u64) -> Result<u16, Error> {
    let rpc_client: &Client = &*RPC_CLIENT;
    let block = get_block(block_height, rpc_client)?;

    let tx_data: Vec<Transaction> = block.txdata;
    Ok(tx_data.len() as u16)
}

fn main() {
    // you can use rpc_client here as if it was a global variable
    // println!("{:?}", res);
    const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes
    dotenv::dotenv().ok();
    let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
    let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    let rpc_password: String =
        env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

    let custom_timeout_transport = jsonrpc::simple_http::Builder::new()
        .url(&rpc_url)
        .expect("invalid rpc url")
        .auth(rpc_user, Some(rpc_password))
        .timeout(TIMEOUT_UTXO_SET_SCANS)
        .build();
    let custom_timeout_rpc_client =
        jsonrpc::client::Client::with_transport(custom_timeout_transport);

    let rpc_client = Client::from_jsonrpc(custom_timeout_rpc_client);
    let res: json::GetTxOutSetInfoResult =
        rpc_client.get_tx_out_set_info(None, None, None).unwrap();
    println!("{:?}", res);

    let num_txn = number_of_transactions(10);
    println!("num_txn={:?}", num_txn);
    let time_to_mine = time_to_mine(15);
    println!("time_to_mine={:?}", time_to_mine);
}
