use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0b4ee9b8a2995dcf")]
pub struct UpdateTreasurySplitTokenAccount {
    pub treasury_split_percentage: u8,
}

pub struct UpdateTreasurySplitTokenAccountInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub treasury_split_token_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateTreasurySplitTokenAccount {
    type ArrangedAccounts = UpdateTreasurySplitTokenAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, treasury_split_token_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTreasurySplitTokenAccountInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            treasury_split_token_account: treasury_split_token_account.pubkey,
        })
    }
}
