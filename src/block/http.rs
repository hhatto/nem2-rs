use serde_json::Value;
use crate::HttpClient;

pub struct BlockHttp<'a> {
    pub client: HttpClient,
    base_path: &'a str,
}

impl<'a> BlockHttp<'a> {
    pub fn new(base_url: &str) -> Self {
        BlockHttp {
            client: HttpClient::new(base_url),
            base_path: "/block",
        }
    }

    pub fn get_block_by_height(self, height: u64) -> Result<Value, String> {
        let path = format!("{}/{}", self.base_path, height);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_block_transactions(self, height: u64) -> Result<Value, String> {
        let path = format!("{}/{}/transactions", self.base_path, height);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_blocks_by_height_with_limit(self, height: u64, limit: usize) -> Result<Value, String> {
        unimplemented!("because path is 'blocks'...");
    }

    pub fn get_merkle_transaction(self, height: u64, hash: &str) -> Result<Value, String> {
        let path = format!("{}/{}/transaction/{}/merkle", self.base_path, height, hash);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }
}
