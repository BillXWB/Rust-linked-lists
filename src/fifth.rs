use std::ptr::null_mut;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}
type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut head) = cur_link {
            cur_link = head.next.take();
        }
    }
}
impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });
        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = null_mut();
            }
            head.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}
impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'this, T> {
    next: Option<&'this Node<T>>,
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}
impl<'this, T> Iterator for Iter<'this, T> {
    type Item = &'this T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'this, T> {
    next: Option<&'this mut Node<T>>,
}
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}
impl<'this, T> Iterator for IterMut<'this, T> {
    type Item = &'this mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6);
        list.push(7);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        assert_eq!(list.peek(), Some(&1));
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        {
            let mut iter = list.iter_mut();
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), Some(&mut 3));
            assert_eq!(iter.next(), None);
        }
        assert_eq!(list.peek(), Some(&1));
    }
}
