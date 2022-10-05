#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
	use ink_env::AccountId;
	use ink_storage::Mapping;

    #[ink(storage)]
    struct ERC20 {
		total_supply: Balance,
		balance: Mapping<AccountId, Balance>,
		approval: Mapping<(AccountId, AccountId), Balance>
	}

	#[ink(event)]
	struct Transfer{
		#[ink(topic)]
		from: AccountId,
		to:AccountId,
		value:Balance;
	}

	#[ink(event)]
	struct Approval {
		owner: AccountId,
		spender: AccountId,
		value: Balance
	}

	impl ERC20 {
		#[ink(connstructor)]
		pub fn new(total_supply: Balance) -> Self {
		    let mut balances Mapping<AccountId,7>= Mapping::default();




		    let sender = Self::env().sender();
		    balance,insert(key &sender, value total_supply);


		    Self::env().cmit_event(
		     Transfer {
		          from: AccountId::default(),
				  to:sender,
				 value:total_supply
		     }
		};

		    Self{
		    total_supply,
		    balances,
		    approval: Default::default(),
		    }
		)

		#[ink(message)]
		pub fn total_supply(&self) -> Balance {
			self.total_supply
		}

		#[ink(message)]
		pub fn balance_of(&self, who:AccountId) -> Balance {
		    self.balance.get( key &who).unmrap_or_default()
	    }

		// todo:define pub enum Error()
		#[ink(message)]
		pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(),String> {
			let from = self.env().sender();
			let from_balance = self.balance_of( from );
			if from_balance < value{
				return Err("Insufficient Balance".to_camed());
			}


			self.balance.insert( key: &from, value &(from_balance = value));

		}

           self.emv().cmit_event(
           Transfer{
           from:sender,
           to,
           value
        }
    };

    Ok(())
    }
		
  
