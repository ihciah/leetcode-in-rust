// Ref: https://leetcode.com/problems/lru-cache/discuss/381773/Rust-safe-linked-list-%2B-hashmap-in-28ms

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;
type Link = Option<NodeRef>;

#[derive(Clone)]
struct Element {
    key: i32,
    val: i32,
}

impl Element {
    fn new(key: i32, val: i32) -> Self {
        Element { key, val }
    }
}

struct Node {
    element: Element,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(element: Element) -> Self {
        Node {
            element,
            prev: None,
            next: None,
        }
    }

    fn into_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }
}

struct List {
    head: Link,
    tail: Link,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, node: NodeRef) {
        node.borrow_mut().prev = self.tail.clone();
        if let Some(tail_ref) = self.tail.clone() {
            tail_ref.borrow_mut().next = Some(node.clone());
        } else {
            self.head = Some(node.clone());
        }
        self.tail = Some(node.clone());
    }

    fn pop_front(&mut self) -> Option<Element> {
        self.head.clone().map(|node_ref| {
            if let Some(next_node_ref) = node_ref.borrow_mut().next.take() {
                next_node_ref.borrow_mut().prev = None;
                self.head = Some(next_node_ref.clone());
            } else {
                self.head = None;
                self.tail = None;
            }
            node_ref.borrow().element.clone()
        })
    }

    fn unlink_node(&mut self, node: NodeRef) -> Element {
        let mut node_ref = node.borrow_mut();
        let prev = node_ref.prev.take();
        let next = node_ref.next.take();

        if let Some(prev_ref) = prev.clone() {
            prev_ref.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next_ref) = next.clone() {
            next_ref.borrow_mut().prev = prev.clone();
        } else {
            self.tail = prev.clone();
        }

        node_ref.element.clone()
    }
}

struct LRUCache {
    map: HashMap<i32, WeakNodeRef>,
    list: List,
    capacity: i32,
    used: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::new(),
            list: List::new(),
            capacity,
            used: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // 1. Get it from map and remove it
        // 2. Unlink it from the list
        // 3. push_back into list and insert into map
        if let Some(element) = self._get_then_remove(key) {
            let val = element.val;
            self._push_then_insert(key, val);
            return val;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        // 1. Get it from map and remove it
        // 2. Unlink it from the list if exists
        // 3. Check if used >= capacity and pop_front from list, remove from map
        // 4. push_back into list and insert into map
        self._get_then_remove(key);
        self._push_then_insert(key, value);
    }

    fn _push_then_insert(&mut self, key: i32, val: i32) {
        while self.used >= self.capacity {
            if let Some(element) = self.list.pop_front() {
                self.map.remove(&element.key);
                self.used -= 1;
            }
        }

        let element = Element::new(key, val);
        let node_ref = Node::new(element).into_ref();
        let weak_node_ref = Rc::downgrade(&node_ref);
        self.list.push_back(node_ref);
        self.map.insert(key, weak_node_ref);
        self.used += 1;
    }

    fn _get_then_remove(&mut self, key: i32) -> Option<Element> {
        if let Some(weak_node_ref) = self.map.remove(&key) {
            if let Some(node_ref) = weak_node_ref.upgrade() {
                let element = self.list.unlink_node(node_ref.clone());
                self.used -= 1;
                return Some(element);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_146() {
        println!("init cache");
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        println!("return 1");
        assert_eq!(lru_cache.get(1), 1); // returns 1
        println!("evict key 2");
        lru_cache.put(3, 3); // evicts key 2
        println!("return -1");
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // evicts key 1
        assert_eq!(lru_cache.get(1), -1); // returns -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // returns 3
        assert_eq!(lru_cache.get(4), 4); // returns 4
    }
}
