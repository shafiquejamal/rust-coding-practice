pub mod tree {
    use std::collections::VecDeque;

    #[derive(Debug, Default)]
    pub struct Tree<T> {
        root: Option<Box<Node<T>>>,
    }

    impl<T: Default + PartialEq> Tree<T> {
        pub fn new() -> Self {
            Tree::default()
        }

        pub fn new_with(data: T) -> Self {
            Tree {
                root: Some(Box::new(Node::new_with(data))),
            }
        }

        pub fn new_with_node(node: Node<T>) -> Self {
            Tree {
                root: Some(Box::new(node)),
            }
        }

        pub fn apply_breadth_first_iterative<U>(&self, f: fn(&Node<T>) -> U) -> Vec<U> {
            let mut q = VecDeque::new();
            if let Some(node) = &self.root {
                q.push_back(node.as_ref());
            }

            let mut results = vec![];
            while let Some(e) = q.pop_front() {
                results.push(f(e));
                for next_child in &e.children {
                    q.push_back(next_child.as_ref())
                }
            }

            results
        }

        pub fn apply_depth_first_iterative<U>(&self, f: fn(&Node<T>) -> U) -> Vec<U> {
            let mut q = VecDeque::new();
            if let Some(node) = &self.root {
                q.push_front(node.as_ref());
            }

            let mut results = vec![];
            while let Some(e) = q.pop_front() {
                results.push(f(e));
                for next_child in (&e.children).iter().rev() {
                    q.push_front(next_child.as_ref())
                }
            }

            results
        }

        pub fn apply_breadth_first_recursive<U>(&self, f: fn(&Node<T>) -> U) -> Vec<U> {
            fn recurse<T, U>(
                mut q: VecDeque<&Node<T>>,
                mut results: Vec<U>,
                f: fn(&Node<T>) -> U,
            ) -> Vec<U> {
                if q.is_empty() {
                    return results;
                } else {
                    let e = q.pop_front().unwrap();
                    results.push(f(e));
                    for c in &e.children {
                        q.push_back(c);
                    }

                    return recurse(q, results, f);
                }
            }
            let mut q = VecDeque::new();
            if let Some(node) = &self.root {
                q.push_back(node.as_ref());
            }
            let results = vec![];
            recurse(q, results, f)
        }

        pub fn apply_depth_first_recursive<U>(&self, f: fn(&Node<T>) -> U) -> Vec<U> {
            fn recurse<T, U>(
                mut q: VecDeque<&Node<T>>,
                mut results: Vec<U>,
                f: fn(&Node<T>) -> U,
            ) -> Vec<U> {
                if q.is_empty() {
                    return results;
                } else {
                    let e = q.pop_front().unwrap();
                    results.push(f(e));
                    for c in (&e.children).iter().rev() {
                        q.push_front(c);
                    }

                    return recurse(q, results, f);
                }
            }
            let mut q = VecDeque::new();
            if let Some(node) = &self.root {
                q.push_front(node.as_ref());
            }
            let results = vec![];
            recurse(q, results, f)
        }

        pub fn level_width(&self) -> Vec<usize> {
            let mut q = Vec::new();
            if let Some(n) = &self.root {
                q.push(n.as_ref())
            }

            let mut results = vec![];
            while !q.is_empty() {
                results.push(q.len());
                q = q
                    .iter()
                    .flat_map(|n| (&*n.children).iter().map(|n| n.as_ref()))
                    .collect::<Vec<&Node<T>>>();
            }
            results
        }
    }

    #[derive(Debug, Default, PartialEq)]
    pub struct Node<T> {
        pub data: T,
        pub children: Vec<Box<Node<T>>>,
    }

    impl<T: Default + PartialEq> Node<T> {
        pub fn new() -> Self {
            Node::default()
        }

        pub fn new_with(data: T) -> Self {
            Node {
                data,
                children: vec![],
            }
        }

        pub fn add(&mut self, data: T) {
            self.children.push(Box::new(Node::new_with(data)));
        }

        pub fn remove(&mut self, data: T) {
            self.children.retain(|c| c.data != data);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::tree::*;

    #[test]
    pub fn can_add_and_remove() {
        let mut node = Node::new_with(10);
        node.add(1);
        node.add(2);
        node.add(3);
        node.remove(2);
        assert_eq!(node.data, 10);
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].data, 1);
        assert_eq!(node.children[1].data, 3);
    }

    #[test]
    pub fn can_traverse_breadth_first_iterative() {
        let mut node = Node::new_with(10);
        node.add(1);
        node.add(2);
        node.add(3);
        node.children[0].add(4);
        node.children[0].add(5);
        node.children[2].add(6);
        let tree = Tree::new_with_node(node);
        let results = tree.apply_breadth_first_iterative(|n| n.data * 2);
        assert_eq!(results, vec![20, 2, 4, 6, 8, 10, 12])
    }

    #[test]
    pub fn can_traverse_breadth_first_recursive() {
        let mut node = Node::new_with(10);
        node.add(1);
        node.add(2);
        node.add(3);
        node.children[0].add(4);
        node.children[0].add(5);
        node.children[2].add(6);
        let tree = Tree::new_with_node(node);
        let results = tree.apply_breadth_first_recursive(|n| n.data * 2);
        assert_eq!(results, vec![20, 2, 4, 6, 8, 10, 12])
    }

    #[test]
    pub fn can_traverse_depth_first_iterative() {
        let mut node = Node::new_with(10);
        node.add(1);
        node.add(2);
        node.add(3);
        node.children[0].add(4);
        node.children[0].add(5);
        node.children[2].add(6);
        let tree = Tree::new_with_node(node);
        let results = tree.apply_depth_first_iterative(|n| n.data * 2);
        assert_eq!(results, vec![20, 2, 8, 10, 4, 6, 12])
    }

    #[test]
    pub fn can_traverse_depth_first_recursive() {
        let mut node = Node::new_with(10);
        node.add(1);
        node.add(2);
        node.add(3);
        node.children[0].add(4);
        node.children[0].add(5);
        node.children[2].add(6);
        let tree = Tree::new_with_node(node);
        let results = tree.apply_depth_first_recursive(|n| n.data * 2);
        assert_eq!(results, vec![20, 2, 8, 10, 4, 6, 12])
    }

    #[test]
    pub fn level_width() {
        let mut node = Node::new_with(10);
        node.add(1);
        node.add(2);
        node.add(3);
        node.add(33);
        node.children[0].add(4);
        node.children[0].add(5);
        node.children[2].add(6);
        let tree = Tree::new_with_node(node);
        let results = tree.level_width();
        assert_eq!(results, vec![1, 4, 3])
    }
}
