#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
#[cfg(not(feature = "ink-experimental-engine"))]
use ink_env::DefaultEnvironment;

#[ink::test]
fn transfer_and_balance() {
    // Set up the PSP34 contract instance with an initial supply of 1000 tokens
    let mut contract = PSP34::new(1000);

    // Check that the initial balance of the account calling the function is 1000
    let caller = DefaultEnvironment::actual_account_id::<ink_env::DefaultEnvironment>().unwrap();
    assert_eq!(contract.balance_of(caller), 1000);

    // Transfer 500 tokens to a different account
    let recipient = ink_env::test::get_account_id::<ink_env::DefaultEnvironment>("recipient");
    assert!(contract.transfer(recipient, 500));

    // Check that the balance of the recipient account is now 500
    assert_eq!(contract.balance_of(recipient), 500);

    // Check that the balance of the original account is now 500
    assert_eq!(contract.balance_of(caller), 500);

    // Test the 'name' function of the contract
    assert_eq!(contract.name(), "My PSP34 Token");

    // Test the 'symbol' function of the contract
    assert_eq!(contract.symbol(), "MPT");
}
