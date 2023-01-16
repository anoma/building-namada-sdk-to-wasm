use tendermint_rpc::{Client as TendermintClient, SimpleRequest, Error};

struct NamadaWebClient;

#[async_trait::async_trait]
impl TendermintClient for NamadaWebClient {
    async fn perform<R>(&self, _request: R) -> Result<R::Response, Error>
    where R: SimpleRequest {
        // we have to utilise the sdk's Client trait implementation here
        Err(Error::client_internal("just for testing".to_string()))
    }
}

fn main() {
    let _namada_web_client = NamadaWebClient;
    println!("Hello, world!");
}
