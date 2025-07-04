use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf326c64cac407f18")]
pub struct CreateNftList {
    pub collection_name: String,
}

pub struct CreateNftListInstructionAccounts {
    pub nft_list: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateNftList {
    type ArrangedAccounts = CreateNftListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nft_list, payer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateNftListInstructionAccounts {
            nft_list: nft_list.pubkey,
            payer: payer.pubkey,
        })
    }
}
