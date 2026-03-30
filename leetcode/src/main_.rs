use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node {
    value: i32,
    next: *mut Node,
}

pub struct LockFreeStack {
    head: AtomicPtr<Node>,
}

impl LockFreeStack {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, value: i32) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut(),
        }));

        loop {
            let current_head = self.head.load(Ordering::Acquire);

            unsafe {
                (*new_node).next = current_head;
            }

            if self.head.compare_exchange(
                current_head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                break;
            }
        }
    }

    pub fn pop(&self) -> Option<i32> {
        loop {
            let current_head = self.head.load(Ordering::Acquire);

            if current_head.is_null() {
                return None;
            }

            let next = unsafe { (*current_head).next };

            if self.head.compare_exchange(
                current_head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                unsafe {
                    let boxed = Box::from_raw(current_head);
                    return Some(boxed.value);
                }
            }
        }
    }
}





fn main() {
    let stack = LockFreeStack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("{:?}", stack.pop()); // Some(30)
    println!("{:?}", stack.pop()); // Some(20)
    println!("{:?}", stack.pop()); // Some(10)
    println!("{:?}", stack.pop()); // None
}