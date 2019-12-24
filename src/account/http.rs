use serde_json::Value;
use crate::HttpClient;

pub struct AccountHttp<'a> {
    pub client: HttpClient,
    base_path: &'a str,
}

impl<'a> AccountHttp<'a> {
    pub fn new(base_url: &str) -> Self {
        AccountHttp {
            client: HttpClient::new(base_url),
            base_path: "/account",
        }
    }

    pub fn get_account_incoming_transactions(self, account_id: &str) -> Result<Value, String> {
        let path = format!("{}/{}/transactions/incoming", self.base_path, account_id);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_account_info(self, account_id: &str) -> Result<Value, String> {
        let path = format!("{}/{}", self.base_path, account_id);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_account_outgoing_transactions(self, account_id: &str) -> Result<Value, String> {
        let path = format!("{}/{}/transactions/outgoing", self.base_path, account_id);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_account_partial_transactions(self, account_id: &str) -> Result<Value, String> {
        let path = format!("{}/{}/transactions/partial", self.base_path, account_id);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_account_transactions(self, account_id: &str) -> Result<Value, String> {
        let path = format!("{}/{}/transactions", self.base_path, account_id);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_account_unconfirmed_transactions(self, account_id: &str) -> Result<Value, String> {
        let path = format!("{}/{}/transactions/unconfirmed", self.base_path, account_id);
        let resp = self.client.get(path.as_str());

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }

    pub fn get_accounts_info(self) -> Result<Value, String> {
        // TODO: status is 409
        let resp = self.client.post(self.base_path);

        if !resp.ok() {
            return Err(format!("error: {:?}", resp));
        }

        Ok(resp.into_json().unwrap())
    }
}
