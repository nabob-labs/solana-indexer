use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd31c3020d7a02317")]
pub struct UpdateRewardFunder {
    pub reward_index: u64,
    pub new_funder: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateRewardFunderInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateRewardFunder {
    type ArrangedAccounts = UpdateRewardFunderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, admin, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateRewardFunderInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
