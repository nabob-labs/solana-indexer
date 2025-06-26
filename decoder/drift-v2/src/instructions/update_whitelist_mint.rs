use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa10fa21394789097")]
pub struct UpdateWhitelistMint {
    pub whitelist_mint: solana_pubkey::Pubkey,
}

pub struct UpdateWhitelistMintInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateWhitelistMint {
    type ArrangedAccounts = UpdateWhitelistMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateWhitelistMintInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
