use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x04000000")]
pub struct AdvanceNonceAccount {}

pub struct AdvanceNonceAccountInstructionAccounts {
    pub nonce_account: solana_pubkey::Pubkey,
    pub recent_blockhashes_sysvar: solana_pubkey::Pubkey,
    pub nonce_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AdvanceNonceAccount {
    type ArrangedAccounts = AdvanceNonceAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nonce_account, recent_blockhashes_sysvar, nonce_authority, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(AdvanceNonceAccountInstructionAccounts {
            nonce_account: nonce_account.pubkey,
            recent_blockhashes_sysvar: recent_blockhashes_sysvar.pubkey,
            nonce_authority: nonce_authority.pubkey,
        })
    }
}
