use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x07dd67996b391bc5")]
pub struct InitializeOpenbookV2FulfillmentConfig {
    pub market_index: u16,
}

pub struct InitializeOpenbookV2FulfillmentConfigInstructionAccounts {
    pub base_spot_market: solana_pubkey::Pubkey,
    pub quote_spot_market: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub openbook_v2_program: solana_pubkey::Pubkey,
    pub openbook_v2_market: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub openbook_v2_fulfillment_config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeOpenbookV2FulfillmentConfig {
    type ArrangedAccounts = InitializeOpenbookV2FulfillmentConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [base_spot_market, quote_spot_market, state, openbook_v2_program, openbook_v2_market, drift_signer, openbook_v2_fulfillment_config, admin, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeOpenbookV2FulfillmentConfigInstructionAccounts {
            base_spot_market: base_spot_market.pubkey,
            quote_spot_market: quote_spot_market.pubkey,
            state: state.pubkey,
            openbook_v2_program: openbook_v2_program.pubkey,
            openbook_v2_market: openbook_v2_market.pubkey,
            drift_signer: drift_signer.pubkey,
            openbook_v2_fulfillment_config: openbook_v2_fulfillment_config.pubkey,
            admin: admin.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
