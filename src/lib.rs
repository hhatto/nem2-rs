use url::Url;
use ureq;
use ureq::Response;

pub mod block;

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
}
