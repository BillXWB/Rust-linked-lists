use std::mem::replace;

pub struct List {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}
impl Drop for List {
    fn drop(&mut self) {
        while let Link::More(_) = self.pop_node() {}
    }
}
impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => Some(node.elem),
        }
    }

    fn pop_node(&mut self) -> Link {
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => Link::Empty,
            Link::More(mut node) => {
                self.head = replace(&mut node.next, Link::Empty);
                Link::More(node)
            }
        }
    }
}
impl Default for List {
    fn default() -> Self {
        Self::new()
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
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
