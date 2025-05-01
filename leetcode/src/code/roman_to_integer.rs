



pub fn roman_to_integer(roman: &str) -> i32 {

    let mut result = 0;
    let mut prev_value = 0;

    for c in roman.chars() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
             _  => 0 // Invalid character
        };
        print!("value {} \n ", value);
        print!("prev_value {}\n ", prev_value);
        print!("result {}\n ", result);

    
        if prev_value < value {
            result -=  prev_value * 2 ;
            print!("Yes\n ");
            // result -= if result > prev_value { prev_value * 2 } else { prev_value };
        }
        result += value;
        prev_value = value;

        print!("result {}\n ", result);
        print!("------------------ \n");

    }
    print!("result {}\n ", result);
   
    result

}