/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
//

use std::cmp::PartialOrd;
use std::marker::Copy;
fn sort<T: PartialOrd + Copy>(array: &mut [T]){
	//TODO
    //Quick Sort
    fn quick_sort<T1: PartialOrd + Copy>(array: &mut [T1], l: usize, r: usize) {
        if l >= r {
            return;
        }

        let x = array[l];
        let mut i = l as i32 - 1;
        let mut j = r as i32 + 1;

        while i < j {
            loop {
                i += 1;
                if array[i as usize] >= x {
                    break;
                }
            }
            
            loop {
                j-=1;
                if array[j as usize] <= x {
                    break;
                }
            }

            if i < j {
                let a = array[i as usize];
                array[i as usize] = array[j as usize];
                array[j as usize] = a;
                //不可用std::mem::swap(&mut array[i as usize], &mut array[j as usize]);
                //因为 cannot borrow `array[_]` as mutable more than once at a time
            } 
        }

        quick_sort(array, l as usize, j as usize);
        quick_sort(array, j as usize + 1, r as usize);
    }
    
    quick_sort::<T>(array, 0, array.len() - 1);
    
    /*  Bubble Sort
    let mut temp;
    for i in 0..(array.len() - 1) {
        for j in 0..(array.len() - 1 - i) {
            if array[j] > array[j + 1] {
                temp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = temp;
            }
        }
    }   */
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
