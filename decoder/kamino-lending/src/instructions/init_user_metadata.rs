use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x75a9b045c5170fa2")]
pub struct InitUserMetadata {
    pub user_lookup_table: solana_sdk::pubkey::Pubkey,
}

pub struct InitUserMetadataInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub user_metadata: solana_sdk::pubkey::Pubkey,
    pub referrer_user_metadata: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitUserMetadata {
    type ArrangedAccounts = InitUserMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, fee_payer, user_metadata, referrer_user_metadata, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitUserMetadataInstructionAccounts {
            owner: owner.pubkey,
            fee_payer: fee_payer.pubkey,
            user_metadata: user_metadata.pubkey,
            referrer_user_metadata: referrer_user_metadata.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
