
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x0f")]
pub struct UpdateMetadataAccountV2{
    pub update_metadata_account_args_v2: UpdateMetadataAccountArgsV2,
}

pub struct UpdateMetadataAccountV2InstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateMetadataAccountV2 {
    type ArrangedAccounts = UpdateMetadataAccountV2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let update_authority = accounts.get(1)?;

        Some(UpdateMetadataAccountV2InstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}