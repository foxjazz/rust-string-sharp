/// returns format of yyyy-mm-dd  
/// 
/// 

use chrono::prelude::*;

pub trait AppendDate{
    fn ymd(self) -> String;
}
impl AppendDate for String{
    fn ymd(self) -> String{
        let mut result = self.clone();
        let now = chrono::Local::now();
        let y = now.year();
        let m = now.month();
        let d = now.day();
        result.push_str(&format!("-{:04}-{:02}-{:02}", y, m, d));
        result
    }
}
/// returns format of hh:mm:ss
pub trait AppendTime{
    fn hms(self) -> String;
}
impl AppendTime for String{
    fn hms(self) -> String{
        let mut result = self.clone();
        let now = chrono::Local::now();
        let h = now.hour();
        let m = now.minute();
        let s = now.second();
        result.push_str(&format!("_{:02}:{:02}:{:02}", h, m, s));
        result
    }
}
