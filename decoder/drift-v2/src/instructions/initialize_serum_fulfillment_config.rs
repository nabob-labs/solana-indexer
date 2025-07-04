use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc1d384ac46ab075e")]
pub struct InitializeSerumFulfillmentConfig {
    pub market_index: u16,
}

pub struct InitializeSerumFulfillmentConfigInstructionAccounts {
    pub base_spot_market: solana_pubkey::Pubkey,
    pub quote_spot_market: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub serum_program: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub serum_open_orders: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub serum_fulfillment_config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeSerumFulfillmentConfig {
    type ArrangedAccounts = InitializeSerumFulfillmentConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [base_spot_market, quote_spot_market, state, serum_program, serum_market, serum_open_orders, drift_signer, serum_fulfillment_config, admin, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeSerumFulfillmentConfigInstructionAccounts {
            base_spot_market: base_spot_market.pubkey,
            quote_spot_market: quote_spot_market.pubkey,
            state: state.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_open_orders: serum_open_orders.pubkey,
            drift_signer: drift_signer.pubkey,
            serum_fulfillment_config: serum_fulfillment_config.pubkey,
            admin: admin.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
