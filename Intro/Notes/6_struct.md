## Struct  -- demonstrating mutable and immutable references . 


```rs

fn main() {
 
    let mut bank : BankAccount = BankAccount {
        owner : String::from("John Doe"),
        balance : 1000.0
    };

    bank.check_balance();
    bank.with_draw(100.0);
    bank.check_balance();
    
}


struct BankAccount {
    owner : String,
    balance : f64
}

impl BankAccount {
    // Mutable borrow to edit balance 
    fn with_draw(&mut self , amount: f64){
        self.balance -= amount
    }

    // Immutable borrow to check balance only 
    fn check_balance(&self ){
        println!("{} has account with balance{}", self.owner , self.balance)
    }
}
```


Above code works because we get one mutable borrow at a time in each function ... once it exits the functions , it's unasigned


