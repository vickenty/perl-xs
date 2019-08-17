use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

// Not public API. Used by generated code.
#[doc(hidden)]
pub struct Registry<T: 'static> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T: 'static> {
    item: T,
    next: Option<&'static Node<T>>,
}

#[doc(hidden)]
impl<T: 'static> Registry<T> {
    // Not public API. Used by generated code.
    pub const fn new() -> Self {
        Registry {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    pub fn submit(&'static self, item: T) {
        let new = Box::leak(Box::new(Node { item, next: None }));

        let mut head = self.head.load(Ordering::SeqCst);
        loop {
            let prev = self.head.compare_and_swap(head, new, Ordering::SeqCst);
            if prev == head {
                // Pointer is always null or valid &'static Node.
                new.next = unsafe { prev.as_ref() };
                return;
            } else {
                head = prev;
            }
        }
    }
    pub fn iter(&self) -> Iter<T> {
        let head = self.head.load(Ordering::SeqCst);
        Iter::<T> {
            // Head pointer is always null or valid &'static Node.
            node: unsafe { head.as_ref() },
        }
    }
}

pub struct Iter<T: 'static> {
    node: Option<&'static Node<T>>,
}

impl<T: 'static> Iterator for Iter<T> {
    type Item = &'static T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        let item = &node.item;
        self.node = node.next;
        Some(item)
    }
}
