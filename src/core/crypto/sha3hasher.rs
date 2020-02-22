use crate::core::crypto::types::SignSchema;
use crate::model::blockchain::types::NetworkType;

pub struct SHA3Hasher {}

impl SHA3Hasher {
    pub fn resolve_sign_schema(network_type: &NetworkType) -> SignSchema {
        match network_type {
            NetworkType::MainNet | NetworkType::TestNet => SignSchema::Keccak,
            _ => SignSchema::Sha3,
        }
    }
}
