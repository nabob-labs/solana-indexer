use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe8fdc3f794d449de")]
pub struct UpdateFee {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}

pub struct UpdateFeeInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateFee {
    type ArrangedAccounts = UpdateFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, fee_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFeeInstructionAccounts {
            keeper: keeper.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
