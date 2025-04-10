fn main() {
   
    
    
    let result = divide(10.0, 0.0);
    match result{
        Ok(x) => println!("Result : {}" , x),
        Err(err) => println!("Error : {}",err)
    };
}


fn divide(num : f64 , den : f64) -> Result<f64 , String>{
    if den == 0.0 {
        return Err("Cannot divide by 0".to_string()); // Return None if denominator is zero
    }else {
        return Ok(num / den); // Return Some with the result
    }
}