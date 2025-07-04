use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x223a39446150f406")]
pub struct DepositIntoPerpMarketFeePool {
    pub amount: u64,
}

pub struct DepositIntoPerpMarketFeePoolInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub source_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub quote_spot_market: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DepositIntoPerpMarketFeePool {
    type ArrangedAccounts = DepositIntoPerpMarketFeePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, admin, source_vault, drift_signer, quote_spot_market, spot_market_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositIntoPerpMarketFeePoolInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            admin: admin.pubkey,
            source_vault: source_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            quote_spot_market: quote_spot_market.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
