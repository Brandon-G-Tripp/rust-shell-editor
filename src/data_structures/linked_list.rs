pub struct Node<T> {
    element: T, 
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
} 

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None, 
            size: 0,
        } 
    } 

    pub fn len(&self) -> usize {
        self.size
    } 

    pub fn is_empty(&self) -> bool {
        self.size == 0
    } 
} 




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    } 


} 
