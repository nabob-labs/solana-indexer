use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x6bcdf9e297235600")]
pub struct SetDefaultProtocolFeeRate {
    pub default_protocol_fee_rate: u16,
}

pub struct SetDefaultProtocolFeeRateInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetDefaultProtocolFeeRate {
    type ArrangedAccounts = SetDefaultProtocolFeeRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, fee_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetDefaultProtocolFeeRateInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
