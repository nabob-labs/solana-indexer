use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x06")]
pub struct DeprecatedCreateReservationList {}

pub struct DeprecatedCreateReservationListInstructionAccounts {
    pub reservation_list: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub resource: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DeprecatedCreateReservationList {
    type ArrangedAccounts = DeprecatedCreateReservationListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reservation_list, payer, update_authority, master_edition, resource, metadata, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DeprecatedCreateReservationListInstructionAccounts {
            reservation_list: reservation_list.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            master_edition: master_edition.pubkey,
            resource: resource.pubkey,
            metadata: metadata.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
