use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf21d86303a6e0e3c")]
pub struct OpenPositionWithMetadata {
    pub bumps: OpenPositionWithMetadataBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

pub struct OpenPositionWithMetadataInstructionAccounts {
    pub funder: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_metadata_account: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub metadata_update_auth: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for OpenPositionWithMetadata {
    type ArrangedAccounts = OpenPositionWithMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [funder, owner, position, position_mint, position_metadata_account, position_token_account, whirlpool, token_program, system_program, rent, associated_token_program, metadata_program, metadata_update_auth, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(OpenPositionWithMetadataInstructionAccounts {
            funder: funder.pubkey,
            owner: owner.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_metadata_account: position_metadata_account.pubkey,
            position_token_account: position_token_account.pubkey,
            whirlpool: whirlpool.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
            metadata_update_auth: metadata_update_auth.pubkey,
        })
    }
}
