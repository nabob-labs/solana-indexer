use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xcf05c8d17a3852b7")]
pub struct SetRewardEmissionsSuperAuthority {}

pub struct SetRewardEmissionsSuperAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
    pub new_reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetRewardEmissionsSuperAuthority {
    type ArrangedAccounts = SetRewardEmissionsSuperAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, reward_emissions_super_authority, new_reward_emissions_super_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetRewardEmissionsSuperAuthorityInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            reward_emissions_super_authority: reward_emissions_super_authority.pubkey,
            new_reward_emissions_super_authority: new_reward_emissions_super_authority.pubkey,
        })
    }
}
