/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
//

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

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

    //get_ith_node中，先定位到linked list上某一节点，然后以该节点为起点，寻找该节点后第index位的数据，
    //由于给定的节点可能不在link list中，或者index超出范围，故为了安全不应将其声明为pub。
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

//  原本未限定merge函数的generic type时无法实现相关目的。另外也可以将T限定为Ord + Clone来调用前面其他的method，实现起来更加简单。
impl<T: std::cmp::Ord> LinkedList<T> {
    pub fn merge(list_a: LinkedList<T>,list_b: LinkedList<T>) -> Self {
                        //TODO
		/*Self {
            length: 0,
            start: None,
            end: None,
        };  */
        let mut list_c = Self::new();
        let mut remain;
        let mut next_c;
        //  Unsafe block for dereferencing raw pointers
        unsafe  {    
            if let Some(next_ptr_a) = list_a.start {
                if let Some(next_ptr_b) = list_b.start {
                    if (*next_ptr_a.as_ptr()).val < (*next_ptr_b.as_ptr()).val {
                        list_c.start = list_a.start;
                        list_c.end = list_a.start;
                        remain = list_b.start;
                        next_c = (*next_ptr_a.as_ptr()).next;
                    } else {
                        list_c.start = list_b.start;
                        list_c.end = list_b.start;
                        remain = list_a.start;
                        next_c = (*next_ptr_b.as_ptr()).next;
                    }
                } else {
                    return list_a;
                }
            } else {
                if list_b.length == 0 {
                    return list_c;
                } else {
                    return list_b;
                }
            }
            while let Some(remain_ptr) = remain {
                match next_c {    
                    Some(next_c_ptr) => {    
                        if (*next_c_ptr.as_ptr()).val < (*remain_ptr.as_ptr()).val {
                            list_c.end = next_c;
                            next_c = (*next_c_ptr.as_ptr()).next;
                        } else {
                            if let Some(next_ptr_c) = list_c.end {
                                (*next_ptr_c.as_ptr()).next = remain;
                                list_c.end = remain;
                                remain = (*remain_ptr.as_ptr()).next;
                                std::mem::swap(&mut next_c, &mut remain);
                            }
                        }
                    }
                    None => {
                        if let Some(next_ptr) = list_c.end {
                            (*next_ptr.as_ptr()).next = remain;
                            list_c.end = remain;
                            break;
                        }
                    }
                }
            }
            while let Some(next_ptr) = next_c {//保证list_c.end为真实的end
                list_c.end = next_c;
                next_c = (*next_ptr.as_ptr()).next;
            }
        }
        list_c.length = list_a.length + list_b.length;
        list_c
    }
}

    /*pub fn merge(list_a: LinkedList<T>,list_b: LinkedList<T>) -> Self {
        //以下是刚开始理解错误，写出的交错合并代码，
        //虽然不符合题意（本题所给的是Ordered List, 也要返回ordered list），但仍有一定价值，故保留。
        //由于空linked list会有None值，后期模式匹配繁琐，不如在这直接排除空串情况。
        if list_a.length == 0 {
            if list_b.length == 0 {
                return Self::new();
            } else {
                return list_b;
            }
        } else if list_b.length == 0 {
            return list_a;
        }

        let list_long;
        let mut list_short;
        if list_a.length > list_b.length {
            list_long = list_a;
            list_short = list_b;
        } else {
            list_long = list_b;
            list_short = list_a;
        }

        let mut list_c = Self::new();
        let mut next: Option<NonNull<Node<T>>> = None;
        if let Some(next_ptr) = list_short.start{
            unsafe{
                next = (*next_ptr.as_ptr()).next;
                (*next_ptr.as_ptr()).next = list_long.start; //list_short的start接入list_long。
            }
        }
        list_c.start = list_short.start;
        list_c.end = list_long.start;
        loop {
            unsafe {
                if let Some(next_ptr) = list_c.end {
                    if let Some(_) = (*next_ptr.as_ptr()).next {
                        std::mem::swap(&mut (*next_ptr.as_ptr()).next, &mut next);
                        list_c.end = (*next_ptr.as_ptr()).next;
                    } else {
                        (*next_ptr.as_ptr()).next = next;
                        list_c.end = list_long.end;
                        list_c.length = list_long.length + list_short.length;
                        break;
                    }
                }
            }
        }
        list_c
        //由于LinkedList并未实现Copy trait, list_a与list_b在传入merge后就会失效。
        //T可能没有实现Copy trait，所以不能直接复制T类型变量，否则会发生ownership move
    }   */

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