// use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

// enum Link {
//     Empty,
//     More(Box<Node>),
// }

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Box::new(Node {
    //         elem: elem,
    //         next: mem::replace(&mut self.head, None),
    //     });

    //     self.head = Some(new_node);
    // }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    // pub fn pop(&mut self) -> Option<i32>{
    //     match mem::replace(&mut self.head, None) {
    //         None => None,
    //         Some(node) => {
    //             self.head = node.next;
    //             Some(node.elem)
    //         }
    //     }
    // }

    pub fn pop(&mut self) -> Option<T> {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        }) 
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoItor<T> {
        IntoItor(self)
    }

    // // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    // //     Iter { next: self.head.map(|node| &node) }
    // // }
    // // 这里我们为 `iter` 声明一个生命周期 'a , 此时 `&self` 需要至少和 `Iter` 活得一样久
    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //     Iter { next: self.head.as_ref().map(|node| &*node) }
    // }
    // pub fn iter<'a>(&'a mut self) -> Iter<'a, T> {
    //     Iter { next: self.head.as_deref() }
    // }
    // pub fn iter(&self) -> Iter<T> {
    //     Iter { next: self.head.as_deref() }
    // }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }

}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }   
}

pub struct IntoItor<T>(List<T>);

impl<T> Iterator for IntoItor<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // Access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.map(|node| {&node});
//             &ndoe.elem
//         })
//     }
    
// }
// 这里声明生命周期是因为下面的关联类型 Item 需要
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // // 这里无需更改，因为上面已经处理了.
    // // Self 依然是这么棒
    // fn next(&mut self) -> Option<Self::Item> {
    //     self.next.map(|node| {
    //         self.next = node.next.as_ref().map(|node| &*node);
    //         &node.elem
    //     })
    // }
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
    
}



// impl Drop for List {
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, None);
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, None);
//         }
//     }
    
// }

// impl List {
//     pub fn new() -> Self {
//         List { head: Link::Empty }
//     }

//     pub fn push(&mut self, elem: i32) {
//         let new_node = Box::new(Node {
//             elem: elem,
//             next: mem::replace(&mut self.head, Link::Empty),
//         });

//         self.head = Link::More(new_node);
//     }

//     pub fn pop(&mut self) -> Option<i32> {
//         match mem::replace(&mut self.head, Link::Empty) {
//             Link::Empty => None,
//             Link::More(node) => {
//                 self.head = node.next;
//                 Some(node.elem)
//             }
//         }
//     }
// }

// impl Drop for List {
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, Link::Empty);

//         while let Link::More(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
//         }
//     }
// }

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
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        // list.peek_mut().map(|&mut value| {
        //     value = 42
        // });
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
    
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
    
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}