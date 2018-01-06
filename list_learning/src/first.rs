use std::mem;
struct Node {
    elem: i32,
    next: Link 
}

pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}


impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(boxed_node) => {
                // set self.head to node.next
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
