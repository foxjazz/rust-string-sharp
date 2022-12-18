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
    pub trait Strip{
        fn strip(self, s: &str) -> String;
    }
    impl Strip for String{
        fn strip(self, s: &str) -> String{
            let result = self.clone();
            result.replace(s, "")
        }
    }

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
        while result != None{
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


