/*
	heap
	This question requires you to implement a binary heap function
*/
//

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
        self.count += 1;
        if self.count == 1 {
            //  items的初始化中会有一个T::default元素，所以初始长度并不为0
            self.items[0] = value;
        } else {
            self.items.push(value);
            //  index使用binary heap中的index值（即索引由1开始递增而非0）
            let mut value_idx = self.count;
            while self.parent_idx(value_idx) > 0 && 
            (self.comparator)(&self.items[value_idx - 1], &self.items[self.parent_idx(value_idx) - 1]) {
                let parent_idx = self.parent_idx(value_idx);
                self.items.swap(value_idx - 1, parent_idx - 1);
                value_idx = self.parent_idx(value_idx);
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
        //  根据测试样例可以知道这里应该返回的是根节点(最小的索引值，不是最小值的索引)， 即“smallest child-index”
        idx
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
        //  将最小值节点与vector尾部节点交换，然后弹出尾部元素，这样容易重组堆，只需要将元素下沉即可。
        //  否则如果是直接弹出最小值节点，或是与其他节点（如头部节点）再将最小值节点弹出，
        //  都会导致二叉树断裂，重新构造二叉树很繁琐。
        if self.count > 0 {
            let mut idx = self.smallest_child_idx(1);
            self.items.swap(idx - 1, self.count - 1);

            self.count -= 1;
            let mut child_idx;
            while self.right_child_idx(idx) < self.len() {
                child_idx = {
                    match (self.comparator)(&self.items[self.left_child_idx(idx) - 1], 
                    &self.items[self.right_child_idx(idx) - 1]) {
                        true => self.left_child_idx(idx),
                        false => self.right_child_idx(idx),
                    }
                };
                if (self.comparator)(&self.items[child_idx - 1], &self.items[idx - 1]) {
                    self.items.swap(child_idx - 1, idx - 1);
                    idx = child_idx;
                } else {
                    break;
                }
            }
            let left_child_idx = self.left_child_idx(idx);
            if self.children_present(idx) && 
            (self.comparator)(&self.items[self.left_child_idx(idx) - 1], &self.items[idx - 1]) {
                self.items.swap(left_child_idx - 1, idx - 1);
            }
            return self.items.pop();
        }
		None
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