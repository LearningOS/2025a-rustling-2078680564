/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
       self.items.push(value);
        self.count += 1;
        // 2. 上浮操作：将新元素调整到堆的正确位置
        let mut current_idx = self.count; // 新元素的索引（有效元素从1开始）
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);
            // 若当前元素与父元素满足比较器条件（最小堆：当前<父；最大堆：当前>父），则交换
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx; // 继续向上比较
            } else {
                break; // 不满足条件，停止上浮
            }
        }
    }
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
      let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 若右孩子存在，比较左右孩子，返回符合比较器的优先子节点
        if right_idx <= self.count {
            if (self.comparator)(&self.items[right_idx], &self.items[left_idx]) {
                right_idx
            } else {
                left_idx
            }
        } else {
            // 仅左孩子存在，返回左孩子索引
            left_idx
        }
    }
}
impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
      if self.is_empty() {
            return None;
        }

        // 1. 移除并获取堆顶元素（索引1），同时将最后一个元素移到堆顶
        let top_element = self.items.swap_remove(1);
        self.count -= 1;

        // 2. 下沉操作：将堆顶元素调整到正确位置
        let mut current_idx = 1;
        while self.children_present(current_idx) {
            let priority_child_idx = self.smallest_child_idx(current_idx);
            // 若当前元素与优先子节点不满足比较器条件，交换两者
            if !(self.comparator)(&self.items[current_idx], &self.items[priority_child_idx]) {
                self.items.swap(current_idx, priority_child_idx);
                current_idx = priority_child_idx; // 继续向下比较
            } else {
                break; // 满足条件，停止下沉
            }
        }

        Some(top_element)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}