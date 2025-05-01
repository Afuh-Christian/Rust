# Variables and Mutability 

- `Variables` are immutable in rust by default
- use `mut` to make them mutable 

```rs 
   let mut _a : i32 = 8 ; 
    _a += 1; 

    println!("The value of a is {}", _a);
    
```


## Constants 

- `const` are by default mutable and can'ts be mad Mutable 

```rs

let mut x = 5;

const Y : i32 = *x - 5;

```

- can be declared anywhere in the app .. 
