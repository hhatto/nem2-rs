use crate::model::blockchain::types::NetworkType;
use crate::core::crypto::sha3hasher::SHA3Hasher;

struct Account {
    pub address: Address,
}

impl Account {
    fn from_private_key(private_key: String, network_type: NetworkType) -> Self {
        let sign_schema = SHA3Hasher::resolve_sign_schema(&network_type);
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
