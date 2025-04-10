### Functions 
```rs 
fn main() {
    let name = "John";
    let age = 30;
    let height = 175;

    human_id(name, age, height);
}


fn human_id(name: &str , age:  u8 , height: u16){
    println!("Name is {} , age is {} , height is {}" , name , age , height);
   }

```

- `Hoisting` - declaring a function anywhere in the code. 


#### Function Returning a value .. 
```rs
fn add(a:i32 , b:i32) -> i32{
    a + b // or return a+b
};

```


### Expressions and Statements .. 

#### Expressions ..
- Returns a value 
e.g 
5
add()
true & false
if else 

```rs

let _X: i32 = {
    let price : i32 = 5;
    let qty : i32 = 10;
    price * qty // (returns price * qty)
} ;


println!("Result : {}" , _X);

```

#### Statement 
- Does not return a value 
let x = 5 ; 
fn foo() {}

#### Global Variables .. 

- Declared with `const` or `static` 











