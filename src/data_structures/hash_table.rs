use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct HashTable<K, V>
where
    K: Clone + Hash + PartialEq,
    V: Clone,
{
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Clone + Hash + PartialEq, 
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        let buckets = vec![Vec::new(); capacity];
        HashTable { buckets, size: 0, }
    } 

    pub fn len(&self) -> usize {
        self.size
    } 

    pub fn capacity(&self) -> usize {
        self.buckets.len()
    } 

    pub fn is_empty(&self) -> bool {
        self.buckets.iter().all(|bucket| bucket.is_empty())
    }

    pub fn insert(&mut self, key: K, value: V) {
        let bucket_index = self.hash(&key);
        let bucket = &mut self.buckets[bucket_index];

        for &mut (ref k, ref mut v) in bucket.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }

        bucket.push((key, value));
        self.size += 1;
    } 

    fn hash(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    } 

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_index = self.hash(key);
        let bucket = &self.buckets[bucket_index];

        for &(ref k, ref v) in bucket.iter() {
            if *k == *key {
                return Some(v);
            }
        } 

        None
    } 

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.hash(key);
        let bucket = &mut self.buckets[bucket_index];

        for i in 0..bucket.len() {
            if bucket[i].0 == *key {
                let (_, value) = bucket.remove(i);
                self.size -= 1;
                return Some(value);
            } 
        } 

        None
    }
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

    #[test]
    fn test_is_empty() {
        let hash_table: HashTable<i32, String> = HashTable::new(10);
        assert!(hash_table.is_empty());

        let mut hash_table2: HashTable<i32, String> = HashTable::new(10);
        hash_table2.buckets[0].push((1, "one".to_string()));
        assert!(!hash_table2.is_empty());
    } 

    #[test]
    fn test_insert() {
        let mut hash_table = HashTable::new(10);

        // Test inserting a new key value pair into empty table
        hash_table.insert(1, "one".to_string());
        assert_eq!(hash_table.len(), 1);
        assert!(!hash_table.is_empty());

        // Test inserting multiple key value pairs into hash table
        hash_table.insert(2, "two".to_string());
        hash_table.insert(3, "three".to_string());
        assert_eq!(hash_table.len(), 3);

        // Test inserting a kv with a key that already exists
        hash_table.insert(2, "updated two".to_string());
        assert_eq!(hash_table.len(), 3);
    } 

    #[test]
    fn test_get() {
        let mut hash_table = HashTable::new(10);

        // Insert some kv pairs
        hash_table.insert(1, "one".to_string());
        hash_table.insert(2, "two".to_string());
        hash_table.insert(3, "three".to_string());

        // Test retrieving value for a key that exists
        assert_eq!(hash_table.get(&1), Some(&"one".to_string()));
        assert_eq!(hash_table.get(&2), Some(&"two".to_string()));
        assert_eq!(hash_table.get(&3), Some(&"three".to_string()));

        // Test retrieving a value for a key 
        assert_eq!(hash_table.get(&4), None);
    } 

    #[test]
    fn test_remove() {
        let mut hash_table = HashTable::new(10);

        // Insert some kv pairs
        hash_table.insert(1, "one".to_string());
        hash_table.insert(2, "two".to_string());
        hash_table.insert(3, "three".to_string());

        assert_eq!(hash_table.remove(&2), Some("two".to_string()));
        assert_eq!(hash_table.len(), 2);
        assert_eq!(hash_table.get(&2), None);

        assert_eq!(hash_table.remove(&4), None);
        assert_eq!(hash_table.len(), 2);
    } 

    #[test]
    fn test_contains_key() {
        let mut hash_table = HashTable::new(10);

        // Insert some kv pairs
        hash_table.insert(1, "one".to_string());
        hash_table.insert(2, "two".to_string());
        hash_table.insert(3, "three".to_string());

        assert!(hash_table.contains_key(&1));
        assert!(hash_table.contains_key(&2));
        assert!(hash_table.contains_key(&3));

        assert!(!hash_table.contains_key(&4));

    } 
} 
