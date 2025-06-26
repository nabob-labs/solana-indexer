use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x06000000")]
pub struct InitializeNonceAccount {
    pub nonce_authority: solana_pubkey::Pubkey,
}

pub struct InitializeNonceAccountInstructionAccounts {
    pub nonce_account: solana_pubkey::Pubkey,
    pub recent_blockhashes_sysvar: solana_pubkey::Pubkey,
    pub rent_sysvar: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeNonceAccount {
    type ArrangedAccounts = InitializeNonceAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nonce_account, recent_blockhashes_sysvar, rent_sysvar, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializeNonceAccountInstructionAccounts {
            nonce_account: nonce_account.pubkey,
            recent_blockhashes_sysvar: recent_blockhashes_sysvar.pubkey,
            rent_sysvar: rent_sysvar.pubkey,
        })
    }
}
