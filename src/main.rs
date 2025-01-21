// use types::AccountId;

mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

#[derive(Debug)]

pub struct Runtime {
    system: system::Pallet<types::AccountId, types::BlockNumber, types::Nonce>,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();

    //first transaction
    /* Execute a transfer from alice to bob for 30 tokens.
    - The transfer _could_ return an error. We should use 'map_err' to print the error if there is one.
    */
    let _ = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("Error: {:?}", e));

    runtime.system.inc_nonce(&alice);

    // second transaction
    let _ = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("Error: {:?}", e));
    // - We should capture the result of the transfer in an unused variable like '
    println!("{:#?}", runtime);
}

// #[test]
// fn init_balances() {
//     let mut balances = balances::Pallet::new();
//     assert_eq!(balances.get_balance(&"alice".to_string()), 0); // Pass a reference
//     balances.set_balance(&"alice".to_string(), 100);
//     assert_eq!(balances.get_balance(&"alice".to_string()), 100); // Pass a reference
//     assert_eq!(balances.get_balance(&"bob".to_string()), 0); // Pass a reference
// }

// #[test]
// fn transfer_balance(){
//     let alice: String = "alice".to_string();
//     let bob: String = "bob".to_string();

//     let mut balances = super::Pallet::new();
//     balances.set_balance(&"alice".to_string(), 100);
//     balances.transfer(caller, to, amount);
// }

// fn fail_test() {
//     assert_eq!(2, 2);
// }
