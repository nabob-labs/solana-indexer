use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x460632f8de018f31")]
pub struct SignedMsgUserOrders {
    pub authority_pubkey: solana_sdk::pubkey::Pubkey,
    pub padding: u32,
    pub signed_msg_order_data: Vec<SignedMsgOrderId>,
}
