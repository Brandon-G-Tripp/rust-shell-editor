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
        assert_eq!(trim(" hello "), "hello");
    } 

    #[test]
    fn test_split() {
        assert_eq!(split("a,b,c", ','), vec!["a", "b", "c"]);
    } 

    #[test]
    fn test_join() {
        assert_eq!(join(&["a", "b", "c"], ","), "a,b,c");
    } 
} 
