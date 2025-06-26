use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x11179fd365b829f1")]
pub struct MigrateBinArray {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateBinArrayInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MigrateBinArray {
    type ArrangedAccounts = MigrateBinArrayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MigrateBinArrayInstructionAccounts {
            lb_pair: lb_pair.pubkey,
        })
    }
}
