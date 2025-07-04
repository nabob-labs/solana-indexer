use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x37ea1052642a7ec0")]
pub struct InitializeOpenOrders {}

pub struct InitializeOpenOrdersInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub open_orders_map: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeOpenOrders {
    type ArrangedAccounts = InitializeOpenOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, dex_program, system_program, open_orders, margin_account, authority, payer, market, serum_authority, open_orders_map, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeOpenOrdersInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            dex_program: dex_program.pubkey,
            system_program: system_program.pubkey,
            open_orders: open_orders.pubkey,
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            market: market.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders_map: open_orders_map.pubkey,
            rent: rent.pubkey,
        })
    }
}
