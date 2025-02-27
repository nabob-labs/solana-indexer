use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x13")]
pub struct InitializeMultisig2 {
    pub m: u8,
}

pub struct InitializeMultisig2InstructionAccounts {
    pub multisig: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMultisig2 {
    type ArrangedAccounts = InitializeMultisig2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [multisig, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMultisig2InstructionAccounts {
            multisig: multisig.pubkey,
        })
    }
}
