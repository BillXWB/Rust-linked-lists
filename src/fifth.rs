pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}
type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}
