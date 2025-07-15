use std::{mem};

pub struct Vec<T> { // 2. nomicon
    ptr: *mut T, cap: usize, // allocation
    len: usize // initialization
}

struct _VecDeque {}











pub struct LinkedList { tail: Link }
enum Link { None, Some(Box<Node>) }
struct Node { e: i32, next: Link }

impl LinkedList {
    pub fn new() -> Self { Self { tail: Link::None } }
    pub fn push(&mut self, val: i32) {
        let n = Box::new(Node { e: val, next: mem::replace(&mut self.tail, Link::None) });
        self.tail = Link::Some(n);
    }

    // NB: return type of .pop() is values of Option<T> rather than references of Box<Node<T>>.
    //     if the interface requires reference semantics then the smart pointer needs to change from Box to Rc,
    //     as the caller *and* data structure need a reference TODO
    pub fn pop(&mut self) -> Option<i32> {
        // match &self.tail {
        //        STACK                                                         HEAP
        //        ___________        
        //        |tail:None|
        //        -----------
        //        Link::None => None

        //        STACK                                                         HEAP
        //        ______________                                                ___________________         _________________
        //        |tail:Some(☐)|----------------------------------------------->|e:9, next:Some(☐)|-------->|e:10, next:None|
        //        --------------                                                -------------------         -----------------
        //        Link::Some(n: &Box<Node>) => self.tail = n.next; Some(n.e)
        // }                                             ^
        //                                               |
        //                                               BC: cannot move out of `node.next` which is behind a shared reference

        // upgrade permissions of the Link from R -> O with mem::replace
        match mem::replace(&mut self.tail, Link::None) {                                                                  
            Link::None => None,
            Link::Some(boxed_n) => {
                self.tail = boxed_n.next; // self.head = node.next (partial move)
                Some(boxed_n.e)
            },
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.tail, Link::None);
        while let Link::Some(mut boxed_n) = cur_link {
            cur_link = mem::replace(&mut boxed_n.next, Link::None)
        }
    }
}

mod test {
    use crate::sequences::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        assert_eq!(list.pop(), None); // check empty
        list.push(1); list.push(2); list.push(3); // populate
        assert_eq!(list.pop(), Some(3)); assert_eq!(list.pop(), Some(2)); // removal
        list.push(4); list.push(5); // populate some more
        assert_eq!(list.pop(), Some(5)); assert_eq!(list.pop(), Some(4)); // remove some more
        assert_eq!(list.pop(), Some(1)); assert_eq!(list.pop(), None); // check exhaustion
    }
}