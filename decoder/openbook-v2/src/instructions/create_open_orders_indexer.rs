use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x404099ffd947f985")]
pub struct CreateOpenOrdersIndexer {}

pub struct CreateOpenOrdersIndexerInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub open_orders_indexer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateOpenOrdersIndexer {
    type ArrangedAccounts = CreateOpenOrdersIndexerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, owner, open_orders_indexer, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateOpenOrdersIndexerInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
