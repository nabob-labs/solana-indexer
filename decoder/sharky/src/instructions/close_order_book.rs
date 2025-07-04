use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdb8649dbb4075ece")]
pub struct CloseOrderBook {}

pub struct CloseOrderBookInstructionAccounts {
    pub order_book: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseOrderBook {
    type ArrangedAccounts = CloseOrderBookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order_book, payer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseOrderBookInstructionAccounts {
            order_book: order_book.pubkey,
            payer: payer.pubkey,
        })
    }
}
