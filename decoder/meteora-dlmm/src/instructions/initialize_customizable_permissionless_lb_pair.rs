use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2e2729876fb7c840")]
pub struct InitializeCustomizablePermissionlessLbPair {
    pub params: CustomizableParams,
}

pub struct InitializeCustomizablePermissionlessLbPairInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub token_mint_x: solana_sdk::pubkey::Pubkey,
    pub token_mint_y: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub user_token_x: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeCustomizablePermissionlessLbPair {
    type ArrangedAccounts = InitializeCustomizablePermissionlessLbPairInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, bin_array_bitmap_extension, token_mint_x, token_mint_y, reserve_x, reserve_y, oracle, user_token_x, funder, token_program, system_program, rent, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            InitializeCustomizablePermissionlessLbPairInstructionAccounts {
                lb_pair: lb_pair.pubkey,
                bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
                token_mint_x: token_mint_x.pubkey,
                token_mint_y: token_mint_y.pubkey,
                reserve_x: reserve_x.pubkey,
                reserve_y: reserve_y.pubkey,
                oracle: oracle.pubkey,
                user_token_x: user_token_x.pubkey,
                funder: funder.pubkey,
                token_program: token_program.pubkey,
                system_program: system_program.pubkey,
                rent: rent.pubkey,
                event_authority: event_authority.pubkey,
                program: program.pubkey,
            },
        )
    }
}
