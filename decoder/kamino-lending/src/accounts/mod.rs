 
use solana_indexer_core::account::AccountDecoder;
use solana_indexer_core::deserialize::IndexerDeserialize;
 

use super::KaminoLendingDecoder; 
pub mod user_state; 
pub mod lending_market; 
pub mod obligation; 
pub mod referrer_state; 
pub mod referrer_token_state; 
pub mod short_url; 
pub mod user_metadata; 
pub mod reserve; 

pub enum KaminoLendingAccount { 
        UserState(user_state::UserState), 
        LendingMarket(lending_market::LendingMarket), 
        Obligation(obligation::Obligation), 
        ReferrerState(referrer_state::ReferrerState), 
        ReferrerTokenState(referrer_token_state::ReferrerTokenState), 
        ShortUrl(short_url::ShortUrl), 
        UserMetadata(user_metadata::UserMetadata), 
        Reserve(reserve::Reserve), 
}


impl<'a> AccountDecoder<'a> for KaminoLendingDecoder { 
    type AccountType = KaminoLendingAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
         
            if let Some(decoded_account) = user_state::UserState::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::UserState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = lending_market::LendingMarket::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::LendingMarket(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = obligation::Obligation::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::Obligation(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = referrer_state::ReferrerState::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::ReferrerState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = referrer_token_state::ReferrerTokenState::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::ReferrerTokenState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = short_url::ShortUrl::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::ShortUrl(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = user_metadata::UserMetadata::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::UserMetadata(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = reserve::Reserve::deserialize(account.data.as_slice()) { 
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports, 
                data: KaminoLendingAccount::Reserve(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}