# Linked Lists in Rust

对着 [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists "mdBook") ~~抄写~~实现了一遍代码。作者是 [The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon "mdBook") 的作者，讲得挺幽默，日常吐槽 rustc，示例代码经常装萌新犯错。“堕落到”使用 _unsafe_ 的时候，场面十分生草：

> And that's that. None of this wimpy reference-counted-dynamic-borrow-checking nonsense! Real. Hard. Unchecked. Pointers.
>
> Let's be C everyone. Let's be C all day.
>
> I'm home. I'm ready.
>
> Hello `unsafe`.

主要实现了 6 个链表：

- [first.rs](src/first.rs "source file")：存储 `i32` 的 naive 单向链表，对外提供栈的接口
- [second.rs](src/second.rs "source file")：泛型单向链表，对外提供栈的接口
- [third.rs](src/third.rs "source file")：泛型可持久化单向链表，对外提供栈的接口
- [fourth.rs](src/fourth.rs "source file")：_safe_ 但功能不全的双向链表，对外提供双端队列的接口
- [fifth.rs](src/fifth.rs "source file")：_unsafe_ 的泛型单向链表，对外提供队列的接口
- [silly1.rs](src/silly1.rs "source file")：基于 `second::List` 实现的 [zipper](<https://en.wikipedia.org/wiki/Zipper_(data_structure)> "Wikipedia")

[第 7 章](https://rust-unofficial.github.io/too-many-lists/sixth.html "mdBook") 的完全体双向链表被咕了… 自己用 `*mut Node<T>` 写了一个，然后跑去看了一下[标准库实现](https://github.com/rust-lang/rust/blob/master/library/alloc/src/collections/linked_list.rs "std::collections::LinkedList")，发现用的是 `Option<NonNull<Node<T>>>`，判空的时候比较方便。但是 `NonNull<T>` 文档说它性质和 `*mut T` 有区别，比如 covariance，不太懂，看标准库里用了一个 `PhantomData` marker，估计就是和这个有关。等看了 [The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon "mdBook") 再补吧:zipper_mouth_face:….​
