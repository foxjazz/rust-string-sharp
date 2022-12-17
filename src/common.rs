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