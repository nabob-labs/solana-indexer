use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x67f9e5e7f7fdc588")]
pub struct CloseOpenOrdersIndexer {}

pub struct CloseOpenOrdersIndexerInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub open_orders_indexer: solana_pubkey::Pubkey,
    pub sol_destination: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseOpenOrdersIndexer {
    type ArrangedAccounts = CloseOpenOrdersIndexerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, open_orders_indexer, sol_destination, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseOpenOrdersIndexerInstructionAccounts {
            owner: owner.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            sol_destination: sol_destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
