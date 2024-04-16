pub fn trim(s: &str) -> &str {
    s.trim()
} 

pub fn split(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
} 

pub fn join(strings: &[&str], separator: &str) -> String {
    strings.join(separator)
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
        assert_eq!(trim("world"), "world");
        assert_eq!(trim("   "), "");
    }

    #[test]
    fn test_split() {
        assert_eq!(split("a,b,c", ','), vec!["a", "b", "c"]);
        assert_eq!(split("hello world", ' '), vec!["hello", "world"]);
        assert_eq!(split("", ','), vec![""]); // Empty string
        assert_eq!(split("no delimiter", 'x'), vec!["no delimiter"]); // Delimiter not found
    }

    #[test]
    fn test_join() {
        assert_eq!(join(&["a", "b", "c"], ","), "a,b,c");
        assert_eq!(join(&["hello", "world"], " "), "hello world");
        assert_eq!(join(&[], ","), ""); // Empty slice
        assert_eq!(join(&["single"], ""), "single"); // Empty separator
    }
} 
