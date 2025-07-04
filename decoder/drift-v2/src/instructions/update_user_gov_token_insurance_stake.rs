use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8f63ebbb149fb854")]
pub struct UpdateUserGovTokenInsuranceStake {}

pub struct UpdateUserGovTokenInsuranceStakeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub insurance_fund_stake: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserGovTokenInsuranceStake {
    type ArrangedAccounts = UpdateUserGovTokenInsuranceStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, insurance_fund_stake, user_stats, signer, insurance_fund_vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdateUserGovTokenInsuranceStakeInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            signer: signer.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
        })
    }
}
