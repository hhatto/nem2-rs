use num::traits::FromPrimitive;

pub enum NetworkType {
    MainNet = 104,
    TestNet = 152,
    Mijin = 96,
    MijinTest = 144,
}

impl FromPrimitive for NetworkType {
    fn from_i64(n: i64) -> Option<NetworkType> {
        match n {
            104 => Some(NetworkType::MainNet),
            152 => Some(NetworkType::TestNet),
            96 => Some(NetworkType::Mijin),
            144 => Some(NetworkType::MijinTest),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<NetworkType> {
        match n {
            104 => Some(NetworkType::MainNet),
            152 => Some(NetworkType::TestNet),
            96 => Some(NetworkType::Mijin),
            144 => Some(NetworkType::MijinTest),
            _ => None,
        }
    }
}
