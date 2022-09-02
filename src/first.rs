// List a = Empty | Elem a (List a)
// A1 -> A2 -> A3 -> Null

// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

// [] = Stack
// () = Heap
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)

// [ptr] -> (Elem A, ptr) -> (Elem B, *null*)


// layout 1:
// [Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)
// split off C:
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
// [Elem C, ptr] -> (Empty *junk*)


// layout 2:
// [ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)
// split off C:
// [ptr] -> (Elem A, ptr) -> (Elem B, *null*)
// [ptr] -> (Elem C, *null*)

// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

// use std::{result, alloc::dealloc};


// List 的尾部不会再分配多余的 junk 值，通过!
// List 枚举的形式可以享受 null 指针优化，完美！
// 所有的元素都拥有统一的内存分配，Good!

pub struct List {
    head: Link,
}

#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // next: self.head.clone(),
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
        // println!("{:?}", self);
    } 

    pub fn pop(&mut self) -> Option<i32> {
    //     let result;
    //     match std::mem::replace(&mut self.head, Link::Empty) {
    //         Link::Empty => {
    //             result = None;
    //         },
    //         Link::More(node) => {
    //             result = Some(node.elem);
    //             self.head = node.next;
    //         }
    //     }

    //     result
    // }
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

// impl Drop for List {
//     fn drop(&mut self) {
//         // NOTE: 在 Rust 代码中，我们不能显式的调用 `drop` 方法，只能调用 std::mem::drop 函数
//         // 这里只是在模拟编译器!
//         // self.head.drop();
//     }
    
// }

// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {},
//             Link::More(ref mut boxed_node) => {
//                 // boxed_node.drop(); // 尾递归 - good!
//             }
//         }
//     }
// }

// impl Drop for Box<Node>{
//     fn drop(&mut self) {
//         // self.ptr.drop(); // 糟糕，这里不是尾递归!
//         // deallocate(self.ptr); // 不是尾递归的原因是在 `drop` 后，还有额外的操作
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link =  std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
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
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }

}