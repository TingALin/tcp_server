#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, 
	ensure, dispatch, traits::Get};
use frame_system::ensure_signed;
use codec::{Decode, Encode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait + generic_asset::Trait {
	type Event: From<Event<Self>> 
	+ Into<<Self as frame_system::Trait>::Event>
	+ Into<<Self as generic_asset::Trait>::Event>;

	type TokenBalance<T>: <T as generic_asset::Trait>::Balance;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Erc20Token<T: Trait> {
	name: Vec<u8>,
	ticker: Vec<u8>,
	total_supply: TokenBalance<T>,
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		TokenIndex get(token_index): u32;
		TokenInfo get(token_info): map u32 => Erc20Token<T>;
		TokenSupply get(token_supply): map u32 => TokenBalance<T>;
		BalanceOf get(balance_of): map (u32, T::AccountId) => T::TokenBalance;
		Allowance get(allowance): map (u32, T::AccountId, T::AccountId) => T::TokenBalance;
	}
}

decl_event!(
	pub enum Event<T> where 
	AccountId = <T as frame_system::Trait>::AccountId,
	TokenBalance = <T as generic_asset::Trait>::Balance,
	 {
		Minted(Vec<u8>, AccountId, TokenBalance),
		Transfer(u32, AccountId, AccountId, TokenBalance),
		Approval(u32, AccountId, AccountId, TokenBalance),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		LowMintedToken,
		TokenExists,
		ItsZero,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
	
		pub fn mint(origin, name:Vec<u8>, ticker:Vec<u8>, total_supply:T::TokenBalance) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(total_supply > 0, Error::<T>::LowMintedToken);
			ensure!(name.len() <= 64, "token name cannot exceed 64 bytes");
			ensure!(ticker.len() <= 32, "token ticker cannot exceed 32 bytes");
			let token_index = Self::token_index();
			ensure!(!TokenInfo::contains_key(token_index), Error::<T>::TokenExists);
			let next_token_id = token_index.checked_add(1).ok_or("overflow on adding token")?;
			<TokenIndex<T>>::put(next_token_id);
			let token = Erc20Token{
				name,
				ticker,
				total_supply,
			};
			<TokenInfo<T>>::insert(next_token_id, token);
			<BalanceOf<T>>::insert(next_token_id, sender), total_supply);
			<TokenSupply<T>>::insert(next_token_id, total_supply);

			Self::deposit_event(RawEvent::Minted(name, sender.clone(),total_supply));					
			Ok(())
		}
		
		fn transfer(origin, token_index:u32, to:T::AccountId, value:T::TokenBalance) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::_transfer(token_index, sender, to, value)
		}

		pub approve(origin, token_index:u32, spender:T::AccountId, value:T::TokenBalance)-> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(<BalanceOf<T>>::exists((token_index, sender.clone())), "Account does not own this token");
			let sender_balance = Self::balance_of((token_index, from.clone()));
			ensure!(sender_balance >= value, "Not enough balance");
			ensure!(value > 0, Error::<T>::ItsZero); 
			let allowance = Self::allowance((token_index, sender.clone(), spender.clone()));
			let updated_allowance = allowance.checked_add(&value).ok_or("overflow in calculating allowance")?;
			<Allowance<T>>::insert((token_index, sender.clone(), spender.clone()), updated_allowance);
  
			Self::deposit_event(RawEvent::Approval(token_index, sender.clone(), spender.clone(), value)); 
			Ok(())
		}

		fn transfer_from(origin, token_index: u32, from: T::AccountId, to: T::AccountId, value: T::TokenBalance) -> dispatch::DispatchResult {
			ensure!(<Allowance<T>>::exists((token_index, from.clone(), to.clone())), "Allowance does not exist.");
			let allowance = Self::allowance((token_index, from.clone(), to.clone()));
			ensure!(allowance >= value, "Not enough allowance.");
			let updated_allowance = allowance.checked_sub(&value).ok_or("overflow in calculating allowance")?;
			<Allowance<T>>::insert((token_index, from.clone(), to.clone()), updated_allowance);
	
			Self::_transfer(token_index, from, to, value)
			Ok(())
		}
	}
}

impl<T:Trait> Module<T> {
	fn _transfer(token_index, from:T::AccountId, to:T::AccountId, value:T::TokenBalance) -> dispatch::DispatchResult{
		ensure!(<BalanceOf<T>>::exists((token_index, from.clone())), "Account does not own this token");
		let sender_balance = Self::balance_of((token_index, from.clone()));
		ensure!(sender_balance >= value, "Not enough balance");
		ensure!(value > 0, Error::<T>::ItsZero);
		let updated_from_balance = ender_balance.checked_sub(&value).ok_or("overflow in calculating balance")?;
		let receiver_balance = Self::balance_of((token_index, to.clone()));
        let updated_to_balance = receiver_balance.checked_add(&value).ok_or("overflow in calculating balance")?;
        <BalanceOf<T>>::insert((token_index, from.clone()), updated_from_balance);
        <BalanceOf<T>>::insert((token_index, to.clone()), updated_to_balance);

        Self::deposit_event(RawEvent::Transfer(token_index, from, to, value));
        Ok(())
	}
}