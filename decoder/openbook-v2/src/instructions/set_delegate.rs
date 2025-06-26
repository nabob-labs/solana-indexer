use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf21e2e4c6ceb80b5")]
pub struct SetDelegate {}

pub struct SetDelegateInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub delegate_account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetDelegate {
    type ArrangedAccounts = SetDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, open_orders_account, delegate_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetDelegateInstructionAccounts {
            owner: owner.pubkey,
            open_orders_account: open_orders_account.pubkey,
            delegate_account: delegate_account.pubkey,
        })
    }
}
