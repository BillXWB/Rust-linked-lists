pub struct List {
    head: Link,
}
impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.pop_node().map(|node| node.elem)
    }

    fn pop_node(&mut self) -> Link {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }
}
impl Drop for List {
    fn drop(&mut self) {
        while let Some(_) = self.pop_node() {}
    }
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
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
