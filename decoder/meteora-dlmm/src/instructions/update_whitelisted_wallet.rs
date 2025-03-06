use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x04695ca7841c095a")]
pub struct UpdateWhitelistedWallet {
    pub wallet: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateWhitelistedWalletInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateWhitelistedWallet {
    type ArrangedAccounts = UpdateWhitelistedWalletInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let creator = accounts.get(1)?;

        Some(UpdateWhitelistedWalletInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            creator: creator.pubkey,
        })
    }
}
