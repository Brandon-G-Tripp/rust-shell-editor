use std::mem;

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

    pub fn push_front(&mut self, element: T) {
        let new_node = Box::new(Node {
            element, 
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    } 

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.element
        })
    }

    pub fn push_back(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: None,
        });

        if let Some(mut last_node) = self.head.as_mut() {
            while let Some(next_node) = mem::take(&mut last_node.next) {
                *last_node = next_node;
            } 
            last_node.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        } 
        self.size += 1;
    } 

    pub fn pop_back(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut node) => {
                if node.next.is_none() {
                    self.size -= 1;
                    Some(node.element)
                } else {
                    let mut current = &mut node;
                    while let Some(ref mut next_node) = current.next {
                        if next_node.next.is_none() {
                            let last_node = next_node.next.take().unwrap();
                            self.size -= 1;
                            return Some(last_node.element);
                        }
                        current = next_node;
                    }
                    self.head = Some(node);
                    None
                }
            }
            None => None,
        }
    } 

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: self.head.as_deref(),
        } 
    } 

    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    } 
} 

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
} 

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.element
        })
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

    #[test]
    fn test_push_front_and_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    } 

    #[test]
    fn test_push_back_and_pop_bac() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    } 

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    } 

    #[test]
    fn test_clear() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.clear();
        assert!(list.is_empty());
    } 
} 
