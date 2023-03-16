use std::{cell::RefCell, rc::Rc, fmt::Debug};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, Default)]
pub struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T, prev: Link<T>, next: Link<T>) -> Self {
        Self { value, prev, next }
    }
}
#[derive(Debug)]
pub struct DLL<T> {
    pub head: Link<T>,
    pub tail: Link<T>,
    length: usize,
}

impl<T> DLL<T>
where T: std::fmt::Debug {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value, None, None)));

        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(new_node);
            self.length += 1;
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
            self.length += 1;
        }
    }

    pub fn unshift(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value, None, None)));

        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::clone(&new_node));
            new_node.borrow_mut().next = Some(head);
            self.head = Some(new_node);
            self.length += 1;
        } else {
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(new_node);
            self.length += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.take() {
            if let Some(prev_tail) = tail.borrow_mut().prev.take() {
                prev_tail.borrow_mut().next = None;
                self.tail = Some(prev_tail);
            } else {
                self.head.take();
            }
            self.length -= 1;
            return Some(Rc::try_unwrap(tail).ok().unwrap().into_inner().value);
        } else {
            None
        }
    }

    pub fn shift(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            if let Some(next_head) = head.borrow_mut().next.take() {
                next_head.borrow_mut().prev = None;
                self.head = Some(next_head);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            return Some(Rc::try_unwrap(head).ok().unwrap().into_inner().value);
        } else {
            None
        }
    }
}

impl<T> Drop for DLL<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

impl<T: Debug> IntoIterator for DLL<T> {
    type Item = T;

    type IntoIter = DLLIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DLLIterator { list: self }
    }
}

pub struct DLLIterator<T> {
    list: DLL<T>,
}

impl<T: Debug> Iterator for DLLIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T: Debug> DoubleEndedIterator for DLLIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.shift()
    }
}
