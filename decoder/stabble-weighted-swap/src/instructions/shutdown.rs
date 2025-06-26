use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x92ccf1d55615fdd3")]
pub struct Shutdown {}

pub struct ShutdownInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Shutdown {
    type ArrangedAccounts = ShutdownInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ShutdownInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
