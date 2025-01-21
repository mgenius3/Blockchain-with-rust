use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub,Zero};



#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet <AccountId, Balance> where  AccountId: Ord + Clone, Balance: CheckedAdd + CheckedSub + Zero + Clone + Copy {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance: Balance = self.get_balance(&caller);
        let to_balance: Balance = self.get_balance(&to);

        let new_caller_balance: Balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient funds")?;

        let new_to_balance: Balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}



#[cfg(test)]
mod tests {

    //configurable types
type AccountId = String;
type Balance = u128;
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();
        assert_eq!(balances.get_balance(&"alice".to_string()), 0); // Pass a reference
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.get_balance(&"alice".to_string()), 100); // Pass a reference
        assert_eq!(balances.get_balance(&"bob".to_string()), 0); // Pass a reference
    }

    #[test]
    fn transfer_balance() {
        let alice: AccountId = "alice".to_string();
        let bob: AccountId = "bob".to_string();

        let mut balances = super::Pallet::new();
        balances.set_balance(&"alice".to_string(), 100);
        let _ = balances.transfer(alice.clone(), bob.clone(), 90);

        assert_eq!(balances.get_balance(&alice), 10);
        assert_eq!(balances.get_balance(&bob), 90);
    }

    #[test]
    fn transfer_balance_insufficient(){
        let alice: AccountId = "alice".to_string();
        let bob: AccountId = "bob".to_string();
        let mut balances = super::Pallet::new();

        balances.set_balance(&"alice".to_string(), 100);

        let result = balances.transfer(alice.clone(), bob.clone(), 110);

        assert_eq!(result, Err("Insufficient funds"));
        assert_eq!(balances.get_balance(&alice), 100);
        assert_eq!(balances.get_balance(&bob),0);
    }

    #[test]
    fn transfer_balance_overflow(){
        let alice: AccountId = "alice".to_string();
        let bob: AccountId = "bob".to_string();
        let mut balances = super::Pallet::new();

        balances.set_balance(&"alice".to_string(), 100);
        balances.set_balance(&"bob".to_string(), Balance::MAX);

        let result = balances.transfer(alice.clone(), bob.clone(), 1);

        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balances.get_balance(&alice), 100);
        assert_eq!(balances.get_balance(&bob), u128::MAX);
    }
}