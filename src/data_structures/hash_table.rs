pub struct HashTable<K, V> {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let capacity = 10;
        let hash_table: HashTable<i32, String> = HashTable::new(capacity);
        assert_eq!(hash_table.len(), 0);
        assert_eq!(hash_table.capacity(), capacity);
    } 
} 
