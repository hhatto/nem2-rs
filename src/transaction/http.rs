use serde_json::Value;
use crate::HttpClient;

#[derive(Clone)]
pub struct TransactionHttp<'a> {
    pub client: HttpClient,
    base_path: &'a str,
}

impl<'a> TransactionHttp<'a> {
    pub fn new(base_url: &str) -> Self {
        TransactionHttp {
            client: HttpClient::new(base_url),
            base_path: "/transaction",
        }
    }

    pub fn get_transaction(self, transaction_id: &str) -> Result<Value, String> {
        let path = format!("{}/{}", self.base_path, transaction_id);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_transactions(self) -> Result<Value, String> {
        unimplemented!("not implement");
    }

    pub fn get_transaction_status(self, hash: &str) -> Result<Value, String> {
        let path = format!("{}/{}/status", self.base_path, hash);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_transactions_statuses(self) -> Result<Value, String> {
        unimplemented!("not implement");
    }

    pub fn announce(self, payload: &str) -> Result<Value, String> {
        if true {
            return Err(format!("not implement"));
        }

        let resp = self.client.put(self.base_path, payload);

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn announce_aggregate_bonded(self) -> Result<Value, String> {
        unimplemented!("not implement");
    }

    pub fn announce_aggregate_bonded_cosignature(self) -> Result<Value, String> {
        unimplemented!("not implement");
    }

    pub fn announce_sync(self) -> Result<Value, String> {
        unimplemented!("not implement");
    }

    pub fn get_transaction_effective_fee(self) -> Result<Value, String> {
        unimplemented!("not implement");
    }
}
