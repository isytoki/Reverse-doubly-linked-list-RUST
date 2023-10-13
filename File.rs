use std::ptr;


struct Node<T> {
    value: T,
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
}


pub struct DoublyLinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: None,
            prev: None,
        }));

        unsafe {
            if let Some(mut old_head) = self.head {
                (*new_node).next = Some(old_head);
                (*old_head).prev = Some(new_node);
            } else {
                self.tail = Some(new_node);
            }

            self.head = Some(new_node);
        }
    }

    pub fn reverse(&mut self) {
        let mut current = self.head;
        let mut new_tail = None;

        while let Some(mut node) = current {
            unsafe {
                let next_node = (*node).next;
                (*node).next = None;
                (*node).prev = new_tail;
                new_tail = Some(node);
                current = next_node;
            }
        }

        // Swap head and tail pointers
        let old_head = self.head;
        self.head = self.tail;
        self.tail = old_head;
    }
}


fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    println!("Original List:");
    for node in list.head.iter() {
        let mut current = Some(*node);
        while let Some(current_node) = current {
            unsafe {
                print!("{} -> ", (*current_node).value);
                current = (*current_node).next;
            }
        }
        println!("NULL");
    }

    list.reverse();

    println!("Reversed List:");
    for node in list.head.iter() {
        let mut current = Some(*node);
        while let Some(current_node) = current {
            unsafe {
                print!("{} -> ", (*current_node).value);
                current = (*current_node).next;
            }
        }
        println!("NULL");
    }
}
