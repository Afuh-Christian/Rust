mod balances;

fn main() {
    let mut pallet = balances::Pallet::new();
    pallet.set_balance("Alice".to_string(), 1000);
    if let Some(balance) = pallet.get_balance(&"Alice".to_string()) {
        println!("Alice's balance is : {}", balance);
    } else {
        println!("Alice has no balance.");
    }

    // println!("Hello, world!");
}
