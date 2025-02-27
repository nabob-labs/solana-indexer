use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x22965df48be1e943")]
pub struct SetCollectProtocolFeesAuthority {}

pub struct SetCollectProtocolFeesAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
    pub new_collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetCollectProtocolFeesAuthority {
    type ArrangedAccounts = SetCollectProtocolFeesAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, collect_protocol_fees_authority, new_collect_protocol_fees_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetCollectProtocolFeesAuthorityInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            collect_protocol_fees_authority: collect_protocol_fees_authority.pubkey,
            new_collect_protocol_fees_authority: new_collect_protocol_fees_authority.pubkey,
        })
    }
}
