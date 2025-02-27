use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9248aee028fd54ae")]
pub struct GoToABin {
    pub bin_id: i32,
}

pub struct GoToABinInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub from_bin_array: solana_sdk::pubkey::Pubkey,
    pub to_bin_array: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for GoToABin {
    type ArrangedAccounts = GoToABinInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, bin_array_bitmap_extension, from_bin_array, to_bin_array, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GoToABinInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            from_bin_array: from_bin_array.pubkey,
            to_bin_array: to_bin_array.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
