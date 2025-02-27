use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2924d8f51b556743")]
pub struct CloseBundledPosition {
    pub bundle_index: u16,
}

pub struct CloseBundledPositionInstructionAccounts {
    pub bundled_position: solana_sdk::pubkey::Pubkey,
    pub position_bundle: solana_sdk::pubkey::Pubkey,
    pub position_bundle_token_account: solana_sdk::pubkey::Pubkey,
    pub position_bundle_authority: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseBundledPosition {
    type ArrangedAccounts = CloseBundledPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [bundled_position, position_bundle, position_bundle_token_account, position_bundle_authority, receiver, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseBundledPositionInstructionAccounts {
            bundled_position: bundled_position.pubkey,
            position_bundle: position_bundle.pubkey,
            position_bundle_token_account: position_bundle_token_account.pubkey,
            position_bundle_authority: position_bundle_authority.pubkey,
            receiver: receiver.pubkey,
        })
    }
}
