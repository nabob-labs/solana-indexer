

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x1e")]
pub struct VerifySizedCollectionItem{
}

pub struct VerifySizedCollectionItemInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition_account: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for VerifySizedCollectionItem {
    type ArrangedAccounts = VerifySizedCollectionItemInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let collection_authority = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let collection_mint = accounts.get(3)?;
        let collection = accounts.get(4)?;
        let collection_master_edition_account = accounts.get(5)?;
        let collection_authority_record = accounts.get(6)?;

        Some(VerifySizedCollectionItemInstructionAccounts {
            metadata: metadata.pubkey,
            collection_authority: collection_authority.pubkey,
            payer: payer.pubkey,
            collection_mint: collection_mint.pubkey,
            collection: collection.pubkey,
            collection_master_edition_account: collection_master_edition_account.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}