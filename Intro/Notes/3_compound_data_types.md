### Array 

 - `&str` is a `string slice` 

```rs
let array : [i32: 4] = [1,2,3 , 5]
let strings : [&str: 3] = ["me","you","her"] // array of strings ..



//display 
println!("Array is : {:?}" , array) // display whole array 
println!("Element is : {}" , array[0]) // display first array 

``` 

`&` means reference . 


### Tuples 
- Collections of elements of fixed size 


```rs
 let human : (String , i32 , bool) = ("Alice".to_string() , 30 , false);
 println!("Human Tuple {:?}" , human);

 // can have arrays inside too .. 
 // can be mixed ... not setting type .. 
```


### Slices

- 

```rs 
let number_slices:&[i32] = &[1,2,3,4]
let animal_slice: &[&String] = &[&"lion".to_string() , &"goat".to_string() , &"elephant".to_string()];

```


### `NB`
- All variables in rust are immutable by default ... add . `mut` to make it mutable .. 

### Strings and String slices (&str) 

#### String 
 - growable
 - Mutable
 - `Owned string types` : They are not borrowed 
 - Allocated on the `heap dynamically ` 
 - slow to access as it's not fixed 

```rs 

let mut new_string: String = String::from("Hello");

new_string.push_str(" , Yes!")

println!("this is : {}" , new_string);

```


#### String Slice
 - Immutable
 - Reference to a portion of string stored somewhere in the code which is good for `memory`
 - Stored on `stack`
 - Good for working with string data without taking ownership of it .
 - Does not copy a value .. just stores it's reference 


```rs

let string: String = String::from("Hello , world");
let slice : &str =  &string; // the slice stores a reference to the string  variable ... 

let slice : &str =  &string[0..5]; // we can also decide the number of character we want .. 

println!("Stored value is : {}" , slice);

```


### NB ..

- Rust cleans any memory allocated to any variable .. (local , global scope stuff .)