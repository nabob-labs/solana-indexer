use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7562427f1e3249b9")]
pub struct InstantCreateTpsl {
    pub params: InstantCreateTpslParams,
}

pub struct InstantCreateTpslInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub api_keeper: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub receiving_account: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub position_request_ata: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub desired_mint: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InstantCreateTpsl {
    type ArrangedAccounts = InstantCreateTpslInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, api_keeper, owner, receiving_account, perpetuals, pool, position, position_request, position_request_ata, custody, custody_doves_price_account, custody_pythnet_price_account, collateral_custody, desired_mint, referral, token_program, associated_token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InstantCreateTpslInstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            receiving_account: receiving_account.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            desired_mint: desired_mint.pubkey,
            referral: referral.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
