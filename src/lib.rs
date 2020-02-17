#[macro_use]
extern crate ureq;

use url::Url;
use ureq::Response;

pub mod core;
pub mod account;
pub mod block;
pub mod transaction;

pub struct Account {
    public_key: String,
    private_key: String,
}

impl Account {
    pub fn from_private_key(private_key: &str) -> Self {
        let public_key = "";
        Account {
            public_key: public_key.to_string(),
            private_key: private_key.to_string(),
        }
    }

    pub fn sign(self, transfer_tx: &TransferTransaction, network_generation_hash: &str) -> SignedTransaction {
        println!("transfer_tx: {:?}, hash: {}, private_key: {}, public_key: {}",
            transfer_tx, network_generation_hash, self.private_key, self.public_key);
        let payload = "".to_string();
        let hash = "".to_string();
        SignedTransaction{payload, hash}
    }
}


#[derive(Debug)]
pub struct TransferTransaction {}

impl TransferTransaction {
    pub fn new() -> Self {
        TransferTransaction {}
    }
}

pub struct SignedTransaction {
    pub payload: String,
    pub hash: String,
}

#[derive(Clone)]
pub struct HttpClient {
    pub base_url: Url,
}


impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            base_url: Url::parse(base_url).unwrap()
        }
    }

    pub fn get(self, path_arg: &str) -> Response {
        let get_url = self.base_url.join(path_arg).unwrap();
        let resp = ureq::get(get_url.as_str())
            .set("Content-Type", "application/json")
            .call();
        resp
    }

    pub fn post(self, path_arg: &str) -> Response {
        let get_url = self.base_url.join(path_arg).unwrap();
        let resp = ureq::post(get_url.as_str())
            .set("Content-Type", "application/json")
            .call();
        resp
    }

    pub fn put(self, path_arg: &str, payload: &str) -> Response {
        let get_url = self.base_url.join(path_arg).unwrap();
        let resp = ureq::put(get_url.as_str())
            .set("Content-Type", "application/json")
            .send_json(json!({"payload": payload}));
        resp
    }
}
