use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d277330ccf62f4239")]
pub struct UpdatePositionOperatorEvent {
    pub position: solana_sdk::pubkey::Pubkey,
    pub old_operator: solana_sdk::pubkey::Pubkey,
    pub new_operator: solana_sdk::pubkey::Pubkey,
}
