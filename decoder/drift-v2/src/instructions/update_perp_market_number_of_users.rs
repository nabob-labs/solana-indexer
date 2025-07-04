use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x233e90b1b43ed7c4")]
pub struct UpdatePerpMarketNumberOfUsers {
    pub number_of_users: Option<u32>,
    pub number_of_users_with_base: Option<u32>,
}

pub struct UpdatePerpMarketNumberOfUsersInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePerpMarketNumberOfUsers {
    type ArrangedAccounts = UpdatePerpMarketNumberOfUsersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketNumberOfUsersInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
