use std::{mem};

// 1.
// linkedlist walkthrough diagram
// reason through the size and alignment (DOD).

// 2.
// informally with aquascope and rust reference
// show how rust's statics (permissions) prevent undefined behavior in dynamics (stack/heap)

// 3.
// formally show this with core calculi and inference rules.

pub struct LinkedList<T> { tail: Link<T> }
type Link<T> = Option<Box<Node<T>>>;
struct Node<T> { e: T, next: Link<T> }

impl<T> LinkedList<T> {
    pub fn new() -> Self { Self { tail: Link::None } }
    pub fn push(&mut self, val: T) {
        let n = Box::new(Node { e: val, next: self.tail.take() });
        self.tail = Link::Some(n);
    }

    // NB: return type of .pop() is values of Option<T> rather than references of Box<Node<T>>,
    pub fn pop(&mut self) -> Option<T> {
        // match &self.tail {
        //     Some(boxed_n) => {
        //         self.tail = boxed_n.next;
        //         Some(boxed_n.e)
        //     },
        //     None => todo!(),
        // }

        // match mem::replace(&mut self.tail, None) {
        //     Some(boxed_n) => {
        //         self.tail = boxed_n.next;
        //         Some(boxed_n.e)
        //     },
        //     None => todo!(),
        // }

        // .take().map() is sugar for indiana jonesing with match mem::replace(&mut dst, None) { .. }
        // which upgrades permissions of self.tail from R -> O
        self.tail.take().map(|boxed_n| {
            self.tail = boxed_n.next; // self.head = node.next (partial move)
            boxed_n.e
        })
    }

    pub fn into_iter(self) -> IntoIter<T> { IntoIter(self) }
    pub fn iter(&self) -> Iter<T> { Iter { next: self.tail.as_deref().map(|boxed_n| &*boxed_n) }}
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.tail, Link::None);
        while let Link::Some(mut boxed_n) = cur_link {
            cur_link = mem::replace(&mut boxed_n.next, Link::None)
        }
    }
}

// 4. informally show lifetimes
// 5. formally show lifetimes
pub struct IntoIter<T>(LinkedList<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { self.0.pop() }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref().map(|boxed_n| &*boxed_n);
            &n.e
        })
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

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3)); assert_eq!(iter.next(), Some(2)); assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}

// 6. make it concurrent