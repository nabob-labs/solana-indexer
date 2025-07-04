use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub swap_fee: u64,
    pub weights: Vec<u64>,
    pub max_caps: Vec<u64>,
}

pub struct InitializeInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, mint, pool, pool_authority, withdraw_authority, vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            owner: owner.pubkey,
            mint: mint.pubkey,
            pool: pool.pubkey,
            pool_authority: pool_authority.pubkey,
            withdraw_authority: withdraw_authority.pubkey,
            vault: vault.pubkey,
        })
    }
}
