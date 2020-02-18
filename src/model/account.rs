use crate::model::blockchain::types::NetworkType;

struct Account {
    pub address: Address,
}

impl Account {
    fn from_private_key(private_key: String, network_type: NetworkType) -> Self {
        Self {
            address: Address {
                address: "".to_string(),
                network_type: network_type,
            }
        }
    }
}

pub struct Address {
    address: String,
    network_type: NetworkType,
}
