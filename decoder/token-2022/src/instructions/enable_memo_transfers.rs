use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1e")]
pub struct EnableMemoTransfers {
    pub memo_transfers_discriminator: u8,
}

pub struct EnableMemoTransfersInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EnableMemoTransfers {
    type ArrangedAccounts = EnableMemoTransfersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableMemoTransfersInstructionAccounts {
            token: token.pubkey,
            owner: owner.pubkey,
        })
    }
}
