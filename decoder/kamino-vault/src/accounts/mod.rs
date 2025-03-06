 
use solana_indexer_core::account::AccountDecoder;
use solana_indexer_core::deserialize::IndexerDeserialize;
 

use super::KaminoVaultDecoder; 
pub mod reserve; 
pub mod vault_state; 

pub enum KaminoVaultAccount { 
        Reserve(reserve::Reserve), 
        VaultState(vault_state::VaultState), 
}


impl<'a> AccountDecoder<'a> for KaminoVaultDecoder { 
    type AccountType = KaminoVaultAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
         
            if let Some(decoded_account) = reserve::Reserve::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoVaultAccount::Reserve(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = vault_state::VaultState::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoVaultAccount::VaultState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}