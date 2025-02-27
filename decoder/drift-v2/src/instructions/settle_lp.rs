use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9be7747161e58b8d")]
pub struct SettleLp {
    pub market_index: u16,
}

pub struct SettleLpInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SettleLp {
    type ArrangedAccounts = SettleLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettleLpInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
        })
    }
}
