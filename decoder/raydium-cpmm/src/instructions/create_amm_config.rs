use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8934edd4d7756c68")]
pub struct CreateAmmConfig {
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
}

pub struct CreateAmmConfigInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateAmmConfig {
    type ArrangedAccounts = CreateAmmConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, amm_config, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateAmmConfigInstructionAccounts {
            owner: owner.pubkey,
            amm_config: amm_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
