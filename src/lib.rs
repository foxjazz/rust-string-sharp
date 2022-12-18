pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod common;
mod string_date;
#[cfg(test)]
mod tests {
    use super::*;
use common::*;
use string_date::AppendDate;
use string_date::AppendTime;

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

    fn test_append_date() {
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.ymd();
        assert_eq!(result, "the quick brown fox jumps over the lazy dog-2022-12-17".to_string());
    }
    #[test]
    fn test_index_of(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.index_of("the");
        assert_eq!(result, 0);
        let s1 = "the quick brown fox jumps over the lazy dog".to_string();
        let result2 = s1.index_of("biggy");
        assert_eq!(result2, -1);
    }
    #[test]
    fn test_indicies_of(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.indicies_of("the");
        assert_eq!(result, 2);
        let s1 = "the quick brown fox jumps over the lazy dog".to_string();
        let result2 = s1.indicies_of("biggy");
        assert_eq!(result2, 0);
    }
}
