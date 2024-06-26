pub struct DynamicArray<T> {
    data: Vec<T>,
} 

impl<T> DynamicArray<T> {
    pub fn new() -> Self {
        DynamicArray { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    } 

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    } 

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    } 

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    } 

    pub fn insert(&mut self, index: usize, item: T) {
        self.data.insert(index, item);
    } 

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    } 

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    } 

    pub fn clear(&mut self) {
        self.data.clear();
    } 
}

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

    #[test]
    fn test_push_and_pop() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.pop(), Some(3));
        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.pop(), None);
        assert!(arr.is_empty());
    } 

    #[test]
    fn test_insert_and_remove() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        arr.insert(1, 4);
        assert_eq!(arr.get(1), Some(&4));
        assert_eq!(arr.remove(2), Some(2));
        assert_eq!(arr.len(), 3);
    }

    #[test] 
    fn test_clear() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        arr.clear();
        assert!(arr.is_empty());
    } 
} 
