pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod common;
#[cfg(test)]
mod tests {
    use super::*;
use common::Append;
use common::Left;
use common::Right;


    #[test]
    fn test_left_and_right() {
        let s = "the quick brown fox jumps over the lazy dog".to_string();

        let left = s.left(3).clone();
        assert_eq!(left, Ok("the".to_string()));
      
    }
    #[test]
    fn test_left(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let left = s.left(3).clone();
        assert_eq!(left, Ok("the".to_string()));
    }
    #[test]
    fn test_right(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let right: Result<String, String> = s.right(3).clone();
        assert_eq!(right, Ok("dog".to_string()));
    }
}
