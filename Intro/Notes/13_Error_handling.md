### Error Handling 

#### Method 1

`OPTION<T>` and enum 

```rs 

enum Option<T>{ // Define the generic Option type 
    Some<T> ,  // Represents a value 
    None // Represents no value 
}; 

```

### Example 

```rs 

fn divide(num : f64 , den : f64) -> Option<f64>{
    if den == 0.0 {
        return None; // Return None if denominator is zero
    }else {
        return Some(num / den); // Return Some with the result
    }
    // return None; // Return None if denominator is zero
    // return Some(num / den); // Return Some with the result
}

// calling the fn 

let result = divide(10.0, 0.0);
match result{
    Some(x) => println!("Result : {}" , x),
    None => println!("Cannot divide by zeror!")
};



```



#### Method 2

`RESULT<T>` and enum - .. for operations that can succeed with OK(T)

```rs 

enum Result<T , E>{ // Define the generic Option type 
    Ok<T> ,  // Represents a value 
    Err<T> // Represents no value 
}; 

// to be able to tell why the operation failed and prevent the error even from compile time 

```


### Example 

```rs 

fn divide(num : f64 , den : f64) -> Result<f64 , String>{
    if den == 0.0 {
        return Err("Cannot divide by 0".to_string()); // Return None if denominator is zero
    }else {
        return Ok(num / den); // Return Some with the result
    }
}


let result = divide(10.0, 0.0);
match result{
    Ok(x) => println!("Result : {}" , x),
    Err(err) => println!("Error : {}",err)
};



```
