use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8080d05bf6351fb0")]
pub struct UpdateFeeParameters {
    pub fee_parameter: FeeParameter,
}

#[derive(Debug, PartialEq)]
pub struct UpdateFeeParametersInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateFeeParameters {
    type ArrangedAccounts = UpdateFeeParametersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, admin, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFeeParametersInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
