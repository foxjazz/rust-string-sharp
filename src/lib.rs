#![crate_name = "string_sharp"]

#![crate_type = "lib"]

mod common;
mod string_date;
#[cfg(test)]
mod tests {
    use super::*;
use common::*;
use string_date::*;


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
        // s.to
        let result = s.indicies_of("the");
        assert_eq!(result, 2);
        let s1 = "the quick brown fox jumps over the lazy dog".to_string();
        let result2 = s1.indicies_of("biggy");
        assert_eq!(result2, 0);
    }

    #[test]
    fn test_trim_end(){
        let s = "the quick brown fox jumps over the lazy dog   ".to_string();
        let result = s.trim_end(' ');
        assert_eq!(result, "the quick brown fox jumps over the lazy dog".to_string());
    }
    #[test]
    fn test_trim_start(){
        let s = "   the quick brown fox jumps over the lazy dog".to_string();
        let result = s.trim_start(' ');
        assert_eq!(result, "the quick brown fox jumps over the lazy dog".to_string());
    }

    #[test]
    fn test_to_upper(){
        let s = "the quick brown fox jumps over the lazy dog".to_string();
        let result = s.to_upper();
        assert_eq!(result, "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG".to_string());
    }
    fn test_to_lower(){
        let s = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG".to_string();
        let result = s.to_lower();
        assert_eq!(result, "the quick brown fox jumps over the lazy dog".to_string());
    }
}
