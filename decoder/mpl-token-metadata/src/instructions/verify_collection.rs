use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x12")]
pub struct VerifyCollection {}

pub struct VerifyCollectionInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub collection_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub collection_master_edition_account: solana_pubkey::Pubkey,
    pub collection_authority_record: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for VerifyCollection {
    type ArrangedAccounts = VerifyCollectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, collection_authority, payer, collection_mint, collection, collection_master_edition_account, collection_authority_record, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(VerifyCollectionInstructionAccounts {
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
