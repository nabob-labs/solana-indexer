use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1b")]
pub struct ApplyConfidentialPendingBalance {
    pub confidential_transfer_discriminator: u8,
    pub expected_pending_balance_credit_counter: u64,
    #[serde(with = "BigArray")]
    pub new_decryptable_available_balance: [u8; 36],
}

pub struct ApplyConfidentialPendingBalanceInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ApplyConfidentialPendingBalance {
    type ArrangedAccounts = ApplyConfidentialPendingBalanceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ApplyConfidentialPendingBalanceInstructionAccounts {
            token: token.pubkey,
            authority: authority.pubkey,
        })
    }
}
