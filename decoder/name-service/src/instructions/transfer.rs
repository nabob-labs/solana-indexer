use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x02")]
pub struct Transfer {
    pub new_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferInstructionAccounts {
    pub name_record: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub parent_name_record: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [name_record, owner, parent_name_record, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferInstructionAccounts {
            name_record: name_record.pubkey,
            owner: owner.pubkey,
            parent_name_record: parent_name_record.pubkey,
        })
    }
}
