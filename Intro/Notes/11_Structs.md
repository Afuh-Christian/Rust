### Structs 
- similar to tuples . 
- both hold multiple related values .
- pieces can hold multiple different types 



### tuple 

```rs 
// tuple
let rect: (i32 , i32) = (200 , 500); 

// Struct 
struct Book {
    title : String , 
    author : String , 
    pages : u32 , 
    available : bool 
}; 

struct User{
    active: bool , 
    username: String , 
    email : String , 
    sign_in_count: u64, 
} ;

// You can instantiate 
// Instance has to be mutable to be able to update .


let mut user1 : User = User{
    active: true , 
    username : String::from("Chris"),
    email: String::from("chris@gmail.com"),
    sign_in_count: 1,
};


user1.email = String::from("afuhchristian@gmail.com");
println!("User email is {}" , user1.email);


// return from functions 


// create instances from other instances 

let mut user2: User = User{
    email: String::from("zinita@gmail.com"),
    ..user1 ,
}



```


#### Tuple Structs

```rs

// Tuple Structs  ... 
// like normal struct ... 

struct Color(i32 , i32 ,i32 )

let black : color = Color(0,0,0)
let white : color = Color(0,0,0)

```
#### Unit-Like struct

```rs
// Unit-Like struct 
// have no fields , used when you need a type  to impl a trait  , but don't need to store data 
struct AlwaysEqual
// instantiate 
let subject: AlwaysEqual = AlwaysEqual

```