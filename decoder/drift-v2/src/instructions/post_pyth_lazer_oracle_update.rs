use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdaedaaf5278fa621")]
pub struct PostPythLazerOracleUpdate {
    pub pyth_message: Vec<u8>,
}

pub struct PostPythLazerOracleUpdateInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub pyth_lazer_storage: solana_pubkey::Pubkey,
    pub ix_sysvar: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PostPythLazerOracleUpdate {
    type ArrangedAccounts = PostPythLazerOracleUpdateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, pyth_lazer_storage, ix_sysvar, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PostPythLazerOracleUpdateInstructionAccounts {
            keeper: keeper.pubkey,
            pyth_lazer_storage: pyth_lazer_storage.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
