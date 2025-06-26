use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7cbd1b2bd8289342")]
pub struct Split {
    pub lamports: u64,
}

pub struct SplitInstructionAccounts {
    pub from: solana_pubkey::Pubkey,
    pub to: solana_pubkey::Pubkey,
    pub stake_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Split {
    type ArrangedAccounts = SplitInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [from, to, stake_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SplitInstructionAccounts {
            from: from.pubkey,
            to: to.pubkey,
            stake_authority: stake_authority.pubkey,
        })
    }
}
