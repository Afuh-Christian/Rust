


pub fn is_palindrome(x: i32) -> bool {
    let mut num : i32  = x ; 
    if num == 0 {
        return true; 
    }else if num > 0 {
    let mut reversed : i32 = 0;
        while num > 0 {
            let last_digit : i32 = num % 10; // get last digit  
            if last_digit == 0 && reversed == 0 {
               return  false;
            }
            reversed = reversed * 10 + last_digit; // introduce new last digit at head . 
            num /= 10; // remove last digit 
        }
        reversed == x
         
    }else {
         false  
    }
}