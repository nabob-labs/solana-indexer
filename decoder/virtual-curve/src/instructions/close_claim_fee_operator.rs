use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x268652d85f7c1163")]
pub struct CloseClaimFeeOperator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseClaimFeeOperatorInstructionAccounts {
    pub claim_fee_operator: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseClaimFeeOperator {
    type ArrangedAccounts = CloseClaimFeeOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [claim_fee_operator, rent_receiver, admin, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseClaimFeeOperatorInstructionAccounts {
            claim_fee_operator: claim_fee_operator.pubkey,
            rent_receiver: rent_receiver.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
