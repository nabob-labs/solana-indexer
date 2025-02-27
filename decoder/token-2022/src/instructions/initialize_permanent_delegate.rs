use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x23")]
pub struct InitializePermanentDelegate {
    pub delegate: solana_sdk::pubkey::Pubkey,
}

pub struct InitializePermanentDelegateInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializePermanentDelegate {
    type ArrangedAccounts = InitializePermanentDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializePermanentDelegateInstructionAccounts { mint: mint.pubkey })
    }
}
