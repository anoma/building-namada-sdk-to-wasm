use tendermint_rpc_only_client::Client as TendermintClient;

struct NamadaClient;

impl TendermintClient for NamadaClient {}

fn main() {
    let _namada_client = NamadaClient;
    println!("Hello, world!");
}
