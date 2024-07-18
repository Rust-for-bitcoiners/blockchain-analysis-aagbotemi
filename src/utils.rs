use bitcoincore_rpc::{bitcoin::Block, Client, Error, RpcApi};

pub fn get_block(block_height: u64, rpc_client: &Client) -> Result<Block, Error> {
    let block_hash = rpc_client.get_block_hash(block_height)?;
    rpc_client.get_block(&block_hash)
}

pub fn time_to_mine_block(difficulty: f64, hash_rate: f64) -> f64 {
    (difficulty * 2.0_f64.powi(32)) / hash_rate
}
