



#[cfg(test)]
mod tests {
    // use super::*;
    use crate::balances;
    #[test]
    fn test_set_and_get_balance() {
        let mut pallet = balances::Pallet::new();
        pallet.set_balance(&"Bob".to_string(), 500);
        assert_eq!(pallet.balance(&"Bob".to_string()), 500);
    }   
    #[test]
    fn test_get_balance_default() {
        let pallet = balances::Pallet::new();
        assert_eq!(pallet.balance(&"Charlie".to_string()), 0);
    }
    #[test]
    fn test_transfer_success() {
        let mut pallet = balances::Pallet::new();
        pallet.set_balance(&"Dave".to_string(), 1000);
        pallet.set_balance(&"Eve".to_string(), 200);
        assert!(pallet.transfer(&"Dave".to_string(), &"Eve".to_string(), 300).is_ok());
        assert_eq!(pallet.balance(&"Dave".to_string()), 700);
        assert_eq!(pallet.balance(&"Eve".to_string()), 500);
    }
}