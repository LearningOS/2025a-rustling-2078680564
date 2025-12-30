/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 辅助方法：将一个节点添加到链表末尾
    fn append_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = None;
        }
        
        match self.end {
            None => {
                self.start = Some(node);
            }
            Some(end_ptr) => {
                unsafe {
                    (*end_ptr.as_ptr()).next = Some(node);
                }
            }
        }
        
        self.end = Some(node);
        self.length += 1;
    }

    // 辅助方法：将剩余的节点添加到链表末尾
    fn append_remaining(&mut self, mut node: NonNull<Node<T>>) {
        // 找到剩余部分的最后一个节点
        let mut last = node;
        while let Some(next) = unsafe { (*last.as_ptr()).next } {
            last = next;
        }

        // 将剩余部分连接到当前链表
        match self.end {
            None => {
                self.start = Some(node);
            }
            Some(end_ptr) => {
                unsafe {
                    (*end_ptr.as_ptr()).next = Some(node);
                }
            }
        }
        
        self.end = Some(last);
        
        // 计算剩余节点数量
        let mut count = 1;
        let mut current = node;
        while let Some(next) = unsafe { (*current.as_ptr()).next } {
            count += 1;
            current = next;
        }
        self.length += count;
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let len_a = list_a.length;
        let len_b = list_b.length;
        let mut merged = LinkedList::new();

        let mut a_ptr = list_a.start.take();
        let mut b_ptr = list_b.start.take();

        // 同时遍历两个链表，每次取较小的节点
        while let (Some(a), Some(b)) = (a_ptr, b_ptr) {
            let a_val = unsafe { &(*a.as_ptr()).val };
            let b_val = unsafe { &(*b.as_ptr()).val };

            if a_val <= b_val {
                // 保存下一个节点
                a_ptr = unsafe { (*a.as_ptr()).next.take() };
                // 将当前节点加入合并链表
                merged.append_node(a);
            } else {
                // 保存下一个节点
                b_ptr = unsafe { (*b.as_ptr()).next.take() };
                // 将当前节点加入合并链表
                merged.append_node(b);
            }
        }

        // 处理剩余节点
        if let Some(a) = a_ptr {
            merged.append_remaining(a);
        }
        if let Some(b) = b_ptr {
            merged.append_remaining(b);
        }

        // 清空原链表
        list_a.length = 0;
        list_a.end = None;
        list_b.length = 0;
        list_b.end = None;

        merged.length = len_a + len_b;
        merged
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1,3,5,7];
        let vec_b = vec![2,4,6,8];
        let target_vec = vec![1,2,3,4,5,6,7,8];
        
        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11,33,44,88,89,90,100];
        let vec_b = vec![1,22,30,45];
        let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
}