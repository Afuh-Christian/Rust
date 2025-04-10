#### OwnerShip 
- Every value in rust has an owner .

```rs 

let s1 = String::from("Hello Christian");

println("Length is : {}" , print_name_length(&s1))


fn print_name_length(s: &String )-> usize {
  s.len()
}

```

- A value can only have one owner at a time . 

```rs 

let v1 : i32 = 8; 
let v2 = v1 ; 

println!("V1 : {}" , v2)
println!("v1 : {}" , v1)

```

`NB`  
>>> Copy Types (No Ownership Move)
Types like `i32, bool, char, f64, and tuples` of Copy types do not move ownership — they are copied instead.

>>> Ownership is a universal concept in Rust:

Strings show it clearly because of heap allocation,

But it works across all types, especially those with non-trivial memory handling (like ```Vec, Box, HashMap, custom structs```, etc.).


- Once a the owner goes out of scope , the value will be dropped 







#### Borrowing and References . 




#### Reference 

`Mutable` - 

```rs

    let mut _x : i32 = 8 ; 
    let _r : &mut i32 = &mut _x ; 
    *_r += 1 ; 
    
    println!("Resulting Reference : {}" , _r)
```

`Immutable` - 

- Without `mut` 




### NB ... You can have only 

- `One mutable reference` 
- Or `Many immutable references`






