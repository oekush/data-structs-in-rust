#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug> LinkedList<T> { // Add the Debug bound here
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.display(); // Output: 3 -> 2 -> 1 -> None
}
