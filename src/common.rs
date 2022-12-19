//! Common functions and traits for the library which has similar functions to string c# library
//! This tool is intended to be used by rust developers coming from a c# background
//! 
/// appends the argument to the string
pub trait Append {
        fn append(self, appender: String) -> String;
}


impl Append for String {
        fn append(self, appender: String) -> String {
            let mut result = self.clone();
            result += appender.as_str();
            result
        }
}

/// returns the leftmost characters of the string
pub trait Left{
    fn left(self, length: usize) -> Result<String, String>;
}
impl Left for String{
    fn left(self, length: usize) -> Result<String, String>{
        if self.len() < length{
           return Err("length arg is higher than string's length".into());
        }
        let ss = self.as_str();
        let ss_new = &ss[0..length];
        Ok(ss_new.to_string())
     }
}

/// returns the rightmost characters of the string
pub trait Right{
    fn right(self, length: usize) -> Result<String, String>;
}
impl Right for String{
    fn right(self, length: usize) -> Result<String, String> 
    {
        if self.len() < length {
            return Err("length arg is higher than string's length".into());
        }
        let ss = self.as_str();
        let begin = self.len() - length;
        let end = begin + length;
        let ss_new = &ss[begin..end];
        let ss_new_string = ss_new.to_string();
        print!("ss_new: {}", ss_new_string);
        Ok(ss_new_string)  
            
    }
        
    
}
/// removes the argument from the string
    pub trait Strip{
        fn strip(self, s: &str) -> String;
    }
    impl Strip for String{
        fn strip(self, s: &str) -> String{
            let result = self.clone();
            result.replace(s, "")
        }
    }
/// strips the characters from the string
pub trait StripChars{
    fn strip_chars(self, chars: &str) -> String;
}
impl StripChars for String{
    fn strip_chars(self, chars: &str) -> String{
        let mut result = self.clone();
        for c in chars.chars(){
            result = result.replace(c, "");
        }
        result
    }
}    
/// reverses the string
pub trait Reverse{
    fn reverse(self) -> String;

}
impl Reverse for String{
    fn reverse(self) -> String{
        let mut result = self.clone();
        result = result.chars().rev().collect();
        result
    }
}
/// finding position of first occurance of a string
pub trait IndexOf{
    fn index_of(self, s: &str) -> i32;
}
impl IndexOf for String{
    fn index_of(self, s: &str) -> i32{
        let new_str = self.as_str();
        let result = new_str.find(s);
        match result{
            Some(i) => i as i32,
            None => -1
        }   
     }
}
/// counting the number of occurances of a string
pub trait IndiciesOf{
    fn indicies_of(self, s: &str) -> i32;
}
impl IndiciesOf for String{
    
    fn indicies_of(self, s: &str) -> i32{
        let len_self = self.len();
        let mut adder = 0;
        let searchlen = s.len();
        let mut idx: usize = 0;
        let new_str = self.as_str();
        let mut result = new_str.find(s);
        if result == None{
            return 0;
        }
        idx = result.unwrap() + searchlen;
        loop{
            adder += 1;
            if idx >= len_self{
                break;
            }
            let nnstr = &new_str[idx..];
            result = nnstr.find(s);
            if result == None{
                break;
            }
            idx += result.unwrap() + searchlen;
        }
        adder

     }

}

/// returns the number of characters in the string
pub trait PadRight{
    fn pad_right(self, length: usize, pad_char: char) -> String;
}
impl PadRight for String{
    fn pad_right(self, length: usize, pad_char: char) -> String{
        let mut result = self.clone();
        let mut adder = length;
        while adder > 0{
            result.push(pad_char);
            adder -= 1;
        }
        result
    }
}
/// pads the string with the character on the left
pub trait PadLeft{
    fn pad_left(self, length: usize, pad_char: char) -> String;
}
impl PadLeft for String{
    fn pad_left(self, length: usize, pad_char: char) -> String{
        let mut result = self.clone();
        let mut adder = length;
        while adder > 0{
            result.insert(0, pad_char);
            adder -= 1;
        }
        result
    }
}
///     
pub trait LastIndexOf{
    fn last_index_of(self, s: &str) -> i32;
}
impl LastIndexOf for String{
    fn last_index_of(self, s: &str) -> i32{
        let new_str = self.as_str();
        let result = new_str.rfind(s);
        match result{
            Some(i) => i as i32,
            None => -1
        }   
     }
}
/// converts the string to lowercase
pub trait ToLower{
    fn to_lower(self) -> String;
}

impl ToLower for String{
    fn to_lower(self) -> String{
        let mut result = self.clone();
        result = result.to_lowercase();
        result
    }
}

/// converts the string to uppercase
pub trait ToUpper{
    fn to_upper(self) -> String;
}
impl ToUpper for String{
    fn to_upper(self) -> String{
        let mut result = self.clone();
        result = result.to_uppercase();
        result
    }
}

/// trims the string from the end
pub trait TrimEnd{
    fn trim_end(self, ch: char) -> String;
}    
impl TrimEnd for String {
    fn trim_end(self, ch: char) -> String{
        let vu8 = ch as u8;
        let result = self.as_str();
        let mut adder = 0;
        let mut idx = result.len() - 1;
        loop{
            if idx < 0{
                break;
            }
            if result.as_bytes()[idx] == vu8{
                adder += 1;
            }
            else{
                break;
            }
            idx -= 1;
        }
        if adder > 0{
            let nidx = result.len() - adder;
            let s = &result[0..nidx];
            return s.to_string();
        }
        result.to_string()
    }
}

/// trims the string from the left
pub trait TrimStart{
    fn trim_start(self, ch: char) -> String;
}
impl TrimStart for String{
    fn trim_start(self, ch: char) -> String{
        let vu8 = ch as u8;
        let result = self.as_str();
        let mut adder = 0;
        let mut idx: usize = 0;
        loop{
            if idx >= result.len(){
                break;
            }
            if result.as_bytes()[idx] == vu8{
                adder += 1;
            }
            else{
                break;
            }
            idx += 1;
        }
        if adder > 0{
            let nidx = result.len();
            let s = &result[adder..nidx];
            return s.to_string();
        }
        result.to_string()
    }
}
    




