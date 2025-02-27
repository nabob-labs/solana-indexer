use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1e")]
pub struct DisableMemoTransfers {
    pub memo_transfers_discriminator: u8,
}

pub struct DisableMemoTransfersInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DisableMemoTransfers {
    type ArrangedAccounts = DisableMemoTransfersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DisableMemoTransfersInstructionAccounts {
            token: token.pubkey,
            owner: owner.pubkey,
        })
    }
}
