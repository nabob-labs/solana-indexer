use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0443514088f55d98")]
pub struct UpdateActive {
    pub stake_index: u32,
    pub validator_index: u32,
}

pub struct UpdateActiveInstructionAccounts {
    pub common: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateActive {
    type ArrangedAccounts = UpdateActiveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [common, validator_list, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateActiveInstructionAccounts {
            common: common.pubkey,
            validator_list: validator_list.pubkey,
        })
    }
}
