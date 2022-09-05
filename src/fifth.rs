// 不错的 unsafe 队列
// use std::mem;

// pub struct List<T> {
//     head: Link<T>,
//     tail: Link<T>, // NEW!
// }pub struct List<'a, T> {


// pub struct List<'a, T> {
//     head: Link<T>,
//     tail: Option<&'a mut Node<T>>, // NEW!
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }
// impl<'a, T> List<'a, T> {
//     pub fn new() -> Self {
//         List { head: None, tail: None }
//     }

//     pub fn push(&'a mut self, elem: T) {
//         let new_tail = Box::new(Node {
//             elem: elem,
//             next: None,
//         });

//         let new_tail = match self.tail.take() {
//             Some(old_tail) => {
//                 old_tail.next = Some(new_tail);
//                 old_tail.next.as_deref_mut()
//             }
//             None => {
//                 self.head = Some(new_tail);
//                 self.head.as_deref_mut()
//             }
//         };

//         self.tail = new_tail;
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head.take().map(|head| {
//             let head = *head;
//             self.head = head.next;

//             if self.head.is_none() {
//                 self.tail = None;
//             }

//             head.elem
//         })
//     }
// }

// mod test {
//     use super::List;
//     #[test]
//     fn basics() {
//         let mut list = List::new();

//         // Check empty list behaves right
//         assert_eq!(list.pop(), None);

//         // Populate list
//         list.push(1);
//         list.push(2);
//         list.push(3);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(1));
//         assert_eq!(list.pop(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push(4);
//         list.push(5);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.pop(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop(), Some(5));
//         assert_eq!(list.pop(), None);
//     }
// // }

// use std::ptr;
// pub struct List<T> {
//     head: Link<T>,
//     tail: *mut Node<T>, // DANGER DANGER
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         List {head: None, tail: ptr::null_mut()}
//     }

//     pub fn push(&mut self, elem: T) {
//         let mut new_tail = Box::new(Node {
//             elem: elem,
//             next: None,
//         });

//         // 将一个普通的引用变成裸指针
//         let raw_tail: *mut _ = &mut *new_tail;
//         // .is_null 会检查是否为 null, 在功能上等价于 `None` 的检查
//         if !self.tail.is_null() {
//             unsafe {
//                 // 如果 old tail 存在，那将其指向新的 tail
//                 (*self.tail).next = Some(new_tail);
//             }
//         } else {
//             self.head = Some(new_tail);
//         }

//         self.tail = raw_tail;
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head.take().map(|head| {
//             let head = *head;
//             self.head = head.next;
    
//             if self.head.is_none() {
//                 self.tail = ptr::null_mut();
//             }
    
//             head.elem
//         })
//     }
    
// }

// #[cfg(test)]
// mod test {
//     use super::List;
    // #[test]
    // fn basics() {
    //     let mut list = List::new();

    //     // Check empty list behaves right
    //     assert_eq!(list.pop(), None);

    //     // Populate list
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);

    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(1));
    //     assert_eq!(list.pop(), Some(2));

    //     // Push some more just to make sure nothing's corrupted
    //     list.push(4);
    //     list.push(5);

    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(3));
    //     assert_eq!(list.pop(), Some(4));

    //     // Check exhaustion
    //     assert_eq!(list.pop(), Some(5));
    //     assert_eq!(list.pop(), None);

    //     // Check the exhaustion case fixed the pointer right
    //     list.push(6);
    //     list.push(7);

    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(6));
    //     assert_eq!(list.pop(), Some(7));
    //     assert_eq!(list.pop(), None);
    // }

    // #[test]
    // fn pointer_aliasing() {
        // let mut data = 10;
        // let ref1 = &mut data;
        // let ref2 = &mut *ref1;

        // *ref2 += 2;
        // *ref1 += 1;

        // let mut data = 10;
        // let ref1 = &mut data;
        // let ref2 = &mut *ref1;

        // // ORDER SWAPPED!
        // *ref1 += 1;
        // *ref2 += 2;

        // println!("{}", data);
        // unsafe {
        //     let mut data = 10;
        //     let ref1 = &mut data;
        //     let ptr2 = ref1 as *mut _;

        //     *ref1 += 1;
        //     *ptr2 += 2;

        //     assert_eq!(data, 13)
        // }

        // unsafe{
        //     let mut data = 10;
        //     let ref1 = &mut data;
        //     let ptr2 = ref1 as *mut _;
        //     let ref3 = &mut *ptr2;
        //     let ptr4 = ref3 as *mut _;

        //     // 首先访问第一个裸指针
        //     // *ptr2 += 2;

        //     // 接着按照借用栈的顺序来访问
        //     *ptr4 += 4;
        //     *ref3 += 3;
        //     *ptr2 += 2;
        //     *ref1 += 1;

        //     assert_eq!(data, 20);
        // }

        // unsafe {
        //     let mut data = [0; 10];
        //     let ref1_at_0 = &mut data[0];
        //     let ptr2_at_0 = ref1_at_0 as *mut i32;
        //     let ptr3_at_1 = ptr2_at_0.add(1);

        //     *ptr3_at_1 += 3;
        //     *ptr2_at_0 += 2;
        //     *ref1_at_0 += 1;

        //     assert_eq!(data, [3, 3, 0, 0, 0, 0, 0, 0, 0, 0])

        // }

        // unsafe {
        //     let mut data = [0; 10];
        //     let ref1_at_0 = &mut data[0];           
        //     let ptr2_at_0 = ref1_at_0 as *mut i32;  
        //     let ptr3_at_0 = ptr2_at_0;            
        
        //     *ptr3_at_0 += 3;
        //     *ptr2_at_0 += 2;
        //     *ref1_at_0 += 1;
        
        //     // Should be [6, 0, 0, ...]
        //     println!("{:?}", &data[..]);
        // }

        // unsafe {
        //     let mut data = [0; 10];
        //     let ref1_at_0 = &mut data[0];            // Reference to 0th element
        //     let ptr2_at_0 = ref1_at_0 as *mut i32;   // Ptr to 0th element
        //     let ptr3_at_0 = ptr2_at_0;               // Ptr to 0th element
        //     let ptr4_at_0 = ptr2_at_0.add(0);        // Ptr to 0th element
        //     let ptr5_at_0 = ptr3_at_0.add(1).sub(1); // Ptr to 0th element
        
        
        //     *ptr3_at_0 += 3;
        //     *ptr2_at_0 += 2;
        //     *ptr4_at_0 += 4;
        //     *ptr5_at_0 += 5;
        //     *ptr3_at_0 += 3;
        //     *ptr2_at_0 += 2;
        //     *ref1_at_0 += 1;
        
        //     // Should be [20, 0, 0, ...]
        //     println!("{:?}", &data[..]);
        // }


    //     //对于部分数据结构，Rust 允许对其中的字段进行独立借用，例如一个结构体，它的多个字段可以被分开借用，来试试这里的数组可不可以。
    //     unsafe {
    //         let mut data = [0; 10];

    //         let slice1_all = &mut data[..];
    //         // let (slice2_at_0, slice_at_1) = slice1.split_at_mut(1);
    //         let ptr2_all = slice1_all.as_mut_ptr();


    //         // let ref1 = &mut slice2_at_0[0];
    //         // let ref2 = &mut slice_at_1[1];
    //         let ptr3 = ptr2_all;
    //         let ptr4 = ptr2_all.add(1);
    //         let ref5 = &mut *ptr3;
    //         let ref6 = &mut *ptr4;

    //         *ref6 += 6;
    //         *ref5 += 5;
    //         *ptr4 += 4;
    //         *ptr3 += 3;

    //         // println!("{:?}", &data[..]);

    //         // 在循环中修改所有元素( 仅仅为了有趣 )
    //         // (可以使用任何裸指针，它们共享同一个借用!)
    //         for idx in 0..10 {
    //             *ptr2_all.add(idx) += idx;
    //         }         

    //         // 同样为了有趣，再实现下安全版本的循环
    //         for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
    //             *elem_ref += idx; 
    //         }

    //         // Should be [8, 12, 4, 6, 8, 10, 12, 14, 16, 18]
    //         println!("{:?}", &data[..]);

    //     }
    // }
    
// }

use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }
    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem: elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}