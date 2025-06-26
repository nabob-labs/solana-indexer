use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5f0704329a4f9c83")]
pub struct SetProtocolFeeRate {
    pub protocol_fee_rate: u16,
}

pub struct SetProtocolFeeRateInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetProtocolFeeRate {
    type ArrangedAccounts = SetProtocolFeeRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, whirlpool, fee_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetProtocolFeeRateInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            whirlpool: whirlpool.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
