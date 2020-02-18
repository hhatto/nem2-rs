use num::traits::FromPrimitive;
use crate::model::blockchain::types::NetworkType;
use crate::model::account::Address;

enum TransactionType {
    Reserved,
    Transfer,
    NamespaceRegistration,
    AddressAlias,
    MosaicAlias,
    MosaicDefinition,
    MosaicSupplyChange,
    MultisigAccountModification,
    AggregateComplete,
    AggregateBonded,
    HashLock,
    SecretLock,
    SecretProof,
    AccountAddressRestriction,
    AccountMosaicRestriction,
    AccountOperationRestriction,
    AccountLink,
    MosaicAddressRestriction,
    MosaicGlobalRestriction,
    AccountMetadata,
    MosaicMetadata,
    NamespaceMetadata,
}

impl FromPrimitive for TransactionType {
    fn from_i64(n: i64) -> Option<TransactionType> {
        match n {
            0 => Some(TransactionType::Reserved),
            16724 => Some(TransactionType::Transfer),
            16718 => Some(TransactionType::NamespaceRegistration),
            16974 => Some(TransactionType::AddressAlias),
            17230 => Some(TransactionType::MosaicAlias),
            16717 => Some(TransactionType::MosaicDefinition),
            16973 => Some(TransactionType::MosaicSupplyChange),
            16725 => Some(TransactionType::MultisigAccountModification),
            16705 => Some(TransactionType::AggregateComplete),
            16961 => Some(TransactionType::AggregateBonded),
            16712 => Some(TransactionType::HashLock),
            16722 => Some(TransactionType::SecretLock),
            16978 => Some(TransactionType::SecretProof),
            16720 => Some(TransactionType::AccountAddressRestriction),
            16976 => Some(TransactionType::AccountMosaicRestriction),
            17232 => Some(TransactionType::AccountOperationRestriction),
            16716 => Some(TransactionType::AccountLink),
            16977 => Some(TransactionType::MosaicAddressRestriction),
            16721 => Some(TransactionType::MosaicGlobalRestriction),
            16708 => Some(TransactionType::AccountMetadata),
            16964 => Some(TransactionType::MosaicMetadata),
            17220 => Some(TransactionType::NamespaceMetadata),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<TransactionType> {
        match n {
            0 => Some(TransactionType::Reserved),
            16724 => Some(TransactionType::Transfer),
            16718 => Some(TransactionType::NamespaceRegistration),
            16974 => Some(TransactionType::AddressAlias),
            17230 => Some(TransactionType::MosaicAlias),
            16717 => Some(TransactionType::MosaicDefinition),
            16973 => Some(TransactionType::MosaicSupplyChange),
            16725 => Some(TransactionType::MultisigAccountModification),
            16705 => Some(TransactionType::AggregateComplete),
            16961 => Some(TransactionType::AggregateBonded),
            16712 => Some(TransactionType::HashLock),
            16722 => Some(TransactionType::SecretLock),
            16978 => Some(TransactionType::SecretProof),
            16720 => Some(TransactionType::AccountAddressRestriction),
            16976 => Some(TransactionType::AccountMosaicRestriction),
            17232 => Some(TransactionType::AccountOperationRestriction),
            16716 => Some(TransactionType::AccountLink),
            16977 => Some(TransactionType::MosaicAddressRestriction),
            16721 => Some(TransactionType::MosaicGlobalRestriction),
            16708 => Some(TransactionType::AccountMetadata),
            16964 => Some(TransactionType::MosaicMetadata),
            17220 => Some(TransactionType::NamespaceMetadata),
            _ => None,
        }
    }
}

struct TransactionInfo {
    height: u64,
    index: u64,
    id: String,
    hash: Option<String>,
    merkle_component_hash: Option<String>,
}

struct Deadline {
    value: time::Duration,
    timestamp_nemesis_block: u64
}

impl Default for Deadline {
    fn default() -> Self {
        Deadline {
            value: time::Duration::hours(2),
            timestamp_nemesis_block: 1573430400,
        }
    }
}

impl Deadline {
    fn new(deadline: Option<time::Duration>) -> Self {
        match deadline {
            Some(v) => Default::default(),
            None => Self { value: time::Duration::hours(2), ..Default::default() }
        }
    }
}

struct PublicAccount {
    public_key: String,
    address: Address,
}

struct Transaction {
    r#type: TransactionType,
    network_type: NetworkType,
    version: usize,
    deadline: Deadline,
    max_fee: u64,
    signature: Option<String>,
    signer: Option<PublicAccount>,
    transaction_info: Option<TransactionInfo>,
}

impl Transaction {
    fn create_transaction_hash(transaction_payload: String, generation_hash: Vec<u8>, network_type: NetworkType) -> String {
        "".to_string()
    }
}
