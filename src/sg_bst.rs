pub mod bst {
    use std::{arch::x86_64::_CMP_TRUE_UQ, fmt::Debug};

    type MaybeNode<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Default)]
    pub struct Node<T> {
        pub data: T,
        pub left: MaybeNode<T>,
        pub right: MaybeNode<T>,
    }

    impl<T> From<Node<T>> for MaybeNode<T> {
        fn from(node: Node<T>) -> Self {
            Some(Box::new(node))
        }
    }

    impl<T: Default + PartialEq + PartialOrd> Node<T> {
        pub fn new() -> Self {
            Node::default()
        }

        pub fn new_with_data(data: T) -> Self {
            Node {
                data,
                left: None,
                right: None,
            }
        }

        pub fn get_data(&self) -> &T {
            &self.data
        }

        pub fn get_left(&self) -> Option<&Node<T>> {
            self.left.as_deref()
        }

        pub fn get_right(&self) -> Option<&Node<T>> {
            self.right.as_deref()
        }

        pub fn insert(&mut self, data: T) {
            if data < self.data {
                match self.left.as_deref_mut() {
                    Some(node) => node.insert(data),
                    None => {
                        self.left = Node::new_with_data(data).into();
                    }
                }
            } else {
                match self.right.as_deref_mut() {
                    Some(node) => node.insert(data),
                    None => {
                        self.right = Node::new_with_data(data).into();
                    }
                }
            }
        }

        pub fn contains(&self, data: T) -> Option<&Node<T>> {
            let mut current = Some(self);
            while let Some(node) = current {
                if node.data == data {
                    return current;
                } else if data < node.data {
                    current = node.left.as_deref();
                } else {
                    current = node.right.as_deref();
                }
            }
            return current;
        }
    }

    pub fn validate<T: PartialEq + PartialOrd + Debug>(node: &Node<T>) -> bool {
        fn recurse<T: PartialEq + PartialOrd + Debug>(
            my_node: &Node<T>,
            min: Option<&T>,
            max: Option<&T>,
        ) -> bool {
            if (min.is_none() || &my_node.data > min.unwrap())
                && (max.is_none() || &my_node.data < max.unwrap())
                && my_node.left.is_none()
                && my_node.right.is_none()
            {
                return true;
            }

            if (min.is_some() && &my_node.data < min.unwrap())
                || (max.is_some() && &my_node.data > max.unwrap())
            {
                return false;
            }

            return (my_node.left.is_none()
                || recurse(my_node.left.as_deref().unwrap(), min, Some(&my_node.data)))
                && (my_node.right.is_none()
                    || recurse(my_node.right.as_deref().unwrap(), Some(&my_node.data), max));
        }

        recurse(node, None, None)
    }
}

#[cfg(test)]
mod tests {

    use super::bst::*;

    #[test]
    fn add_contains() {
        let mut node = Node::new_with_data(10);
        node.insert(5);
        node.insert(11);
        node.insert(4);
        node.insert(6);
        node.insert(10);
        node.insert(12);
        assert_eq!(node.get_data(), &10);
        assert_eq!(node.get_left().unwrap().get_data(), &5);
        assert_eq!(node.get_right().unwrap().get_data(), &11);
        assert_eq!(node.get_left().unwrap().get_left().unwrap().get_data(), &4);
        assert_eq!(node.get_left().unwrap().get_right().unwrap().get_data(), &6);
        assert_eq!(
            node.get_right().unwrap().get_left().unwrap().get_data(),
            &10
        );
        assert_eq!(
            node.get_right().unwrap().get_right().unwrap().get_data(),
            &12
        );
        assert_eq!(node.contains(10).unwrap().get_data(), &10);
        assert_eq!(node.contains(12).unwrap().get_data(), &12);
        assert!(node.contains(13).is_none());
        assert!(node.contains(13).is_none());
        assert_eq!(node.contains(5).unwrap().get_data(), &5);
        assert!(node
            .get_left()
            .unwrap()
            .get_left()
            .unwrap()
            .contains(5)
            .is_none());
    }

    #[test]
    fn validate_bst() {
        let mut node = Node::new_with_data(20);
        assert!(validate(&node));
        node.insert(10);
        node.insert(25);
        assert!(validate(&node));
        node.insert(6);
        node.insert(12);
        node.insert(23);
        node.insert(30);
        node.insert(2);
        node.insert(8);
        node.insert(21);
        node.insert(24);
        assert!(validate(&node));
        node.left
            .as_deref_mut()
            .unwrap()
            .right
            .as_deref_mut()
            .unwrap()
            .data = 21;
        assert!(!validate(&node));
    }
}
