use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7e35b00f276761f3")]
pub struct FlashTakeOrderStart {
    pub input_amount: u64,
    pub min_output_amount: u64,
    pub tip_amount_permissionless_taking: u64,
}

pub struct FlashTakeOrderStartInstructionAccounts {
    pub taker: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub pda_authority: solana_pubkey::Pubkey,
    pub order: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub input_vault: solana_pubkey::Pubkey,
    pub taker_input_ata: solana_pubkey::Pubkey,
    pub taker_output_ata: solana_pubkey::Pubkey,
    pub intermediary_output_token_account: solana_pubkey::Pubkey,
    pub maker_output_ata: solana_pubkey::Pubkey,
    pub express_relay: solana_pubkey::Pubkey,
    pub express_relay_metadata: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub permission: solana_pubkey::Pubkey,
    pub config_router: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
    pub output_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for FlashTakeOrderStart {
    type ArrangedAccounts = FlashTakeOrderStartInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [taker, maker, global_config, pda_authority, order, input_mint, output_mint, input_vault, taker_input_ata, taker_output_ata, intermediary_output_token_account, maker_output_ata, express_relay, express_relay_metadata, sysvar_instructions, permission, config_router, input_token_program, output_token_program, system_program, rent, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(FlashTakeOrderStartInstructionAccounts {
            taker: taker.pubkey,
            maker: maker.pubkey,
            global_config: global_config.pubkey,
            pda_authority: pda_authority.pubkey,
            order: order.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            input_vault: input_vault.pubkey,
            taker_input_ata: taker_input_ata.pubkey,
            taker_output_ata: taker_output_ata.pubkey,
            intermediary_output_token_account: intermediary_output_token_account.pubkey,
            maker_output_ata: maker_output_ata.pubkey,
            express_relay: express_relay.pubkey,
            express_relay_metadata: express_relay_metadata.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            permission: permission.pubkey,
            config_router: config_router.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
