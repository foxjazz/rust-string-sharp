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
use common::Strip;
use common::StripChars;
use common::Reverse;

    #[test]
    fn test_append() {
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.append(" and the cat".to_string());
        assert_eq!(result, "the quick brown fox jumps over the lazy dog and the cat".to_string());
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
    #[test]
    fn test_strip(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.strip("the");
        assert_eq!(result, " quick brown fox jumps over  lazy dog".to_string());
    }
    #[test]
    fn test_strip_chars(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.strip_chars("the");
        assert_eq!(result, " quick brown fox jumps ovr  lazy dog".to_string());
    }
    #[test]
    fn test_reverse(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.reverse();
        assert_eq!(result, "god yzal eht revo spmuj xof nworb kciuq eht".to_string());
    }

}
