
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    //   let mut result = String::new();
    let mut prefix = String::new();
        if strs.is_empty() {
            return prefix;
        }
        let first_str = &strs[0];

        for char in first_str.chars() {
            let mut present_char = false;
            prefix.push(char);
            for str in strs.iter(){
                if str.starts_with(&prefix) {
                    present_char = true;
                    // continue;
                }else {
                    present_char = false;
                    break;
                }
            }
            if(present_char == false){
                prefix.pop(); // remove last char from prefix 
                break;
            }

        }
        prefix
}


pub fn longest_common_prefix_(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first_str = &strs[0];
    let mut prefix = String::new();



    for (i, ch) in first_str.chars().enumerate() {
        for s in strs.iter() {
            // if the string is too short or the character at position i doesn't match
            if s.len() <= i || s.chars().nth(i) != Some(ch) {
                return prefix;
            }
        }
        prefix.push(ch);
    }

    prefix
}
