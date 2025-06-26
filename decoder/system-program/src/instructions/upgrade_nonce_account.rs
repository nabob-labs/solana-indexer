use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0c000000")]
pub struct UpgradeNonceAccount {}

pub struct UpgradeNonceAccountInstructionAccounts {
    pub nonce_account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpgradeNonceAccount {
    type ArrangedAccounts = UpgradeNonceAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nonce_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpgradeNonceAccountInstructionAccounts {
            nonce_account: nonce_account.pubkey,
        })
    }
}
