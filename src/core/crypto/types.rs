use num::traits::FromPrimitive;

pub enum SignSchema {
    Keccak,
    Sha3,
}

impl FromPrimitive for SignSchema {
    fn from_i64(n: i64) -> Option<SignSchema> {
        match n {
            1 => Some(SignSchema::Keccak),
            2 => Some(SignSchema::Sha3),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<SignSchema> {
        match n {
            1 => Some(SignSchema::Keccak),
            2 => Some(SignSchema::Sha3),
            _ => None,
        }
    }
}
