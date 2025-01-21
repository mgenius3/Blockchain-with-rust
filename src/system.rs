use num::{traits::{CheckedAdd, CheckedSub, Zero}, One};
use std::{collections::BTreeMap, default, ops::AddAssign};

// use crate::types::AccountId;

//configurable types
// type AccountId = String;
// type BlockNumber = u32;
// type Nonce = u32;

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: CheckedAdd + CheckedSub + Zero + Clone + Copy + One + AddAssign,
    Nonce: CheckedAdd + CheckedSub + Zero + Clone + Copy + One,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        //crashes if overflow
        self.block_number += BlockNumber::one(); 
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce: Nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who.clone(), nonce + Nonce::one());

        // unimplemented!()
    }

    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }
}

mod test {

    fn inc_block_number() {}

    #[test]

    fn inc_nonce() {
        let alice = String::from("alice");
        let mut system :super::Pallet<String, u32, u32> = super::Pallet::new();
        system.inc_nonce(&alice.clone());
        assert_eq!(system.get_nonce(&alice), 1)
    }
}
