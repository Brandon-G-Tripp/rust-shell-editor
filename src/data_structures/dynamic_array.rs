#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr: DynamicArray<i32> = DynamicArray::new();
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 0);
        assert!(arr.is_empty());
    } 
} 
