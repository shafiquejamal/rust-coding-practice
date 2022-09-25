pub mod linked_list {
    use std::fmt::Debug;

    type MaybeNode<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Default, PartialEq)]
    pub struct Node<T> {
        data: T,
        next: MaybeNode<T>,
    }

    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<T: Default> Node<T> {
        pub fn new() -> Self {
            Node::<T>::default()
        }

        pub fn new_with(data: T) -> Self {
            Node { data, next: None }
        }

        pub fn set_next(&mut self, next: MaybeNode<T>) {
            self.next = next
        }
    }

    impl<T> From<Node<T>> for MaybeNode<T> {
        fn from(node: Node<T>) -> Self {
            Some(Box::new(node))
        }
    }

    #[derive(Debug, Default)]
    pub struct LinkedList<T> {
        head: MaybeNode<T>,
    }

    impl<T: Default + Debug> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList::<T>::default()
        }

        pub fn insert_first(&mut self, data: T) {
            let mut node = Node::new_with(data);
            node.next = self.head.take();
            self.head = node.into();
        }

        pub fn get_first(&self) -> Option<&Node<T>> {
            self.head.as_deref()
        }

        pub fn iter(&self) -> Iter<T> {
            Iter {
                next: self.head.as_deref(),
            }
        }

        pub fn get_last(&self) -> Option<&Node<T>> {
            self.iter().last()
        }

        pub fn get_size(&self) -> usize {
            let mut count: usize = 0;
            for _ in self.iter() {
                count += 1
            }
            count
        }

        pub fn clear(&mut self) {
            self.head = None
        }

        pub fn remove_first(&mut self) -> Option<Node<T>> {
            self.head.take().map(|mut n| {
                self.head = n.next.take();
                *n
            })
        }

        pub fn remove_last(&mut self) -> Option<Node<T>> {
            let mut current = &mut self.head;
            loop {
                match current {
                    None => return None,
                    Some(n) if n.next.as_deref().is_none() => {
                        return Some(*current.take().unwrap());
                    }
                    Some(node) => current = &mut node.next,
                }
            }
        }

        pub fn get_at(&self, index: usize) -> Option<&Node<T>> {
            if index < 0 {
                return None;
            }
            let mut count = 0;
            let mut current = self.head.as_deref();
            while let Some(node) = current {
                if count == index {
                    return current;
                }
                current = node.next.as_deref();
                count += 1;
            }
            return None;
        }

        pub fn get_middle(&self) -> Option<&Node<T>> {
            if self.head.is_none() {
                return None;
            }

            let mut slow = self.head.as_deref().unwrap();
            let mut fast = self.head.as_deref().unwrap();
            while fast.next.is_some() && fast.next.as_deref().unwrap().next.is_some() {
                slow = slow.next.as_deref().unwrap();
                fast = fast.next.as_deref().unwrap().next.as_deref().unwrap();
            }

            Some(slow)
        }

        pub fn get_from_last(&self, k: usize) -> Option<&Node<T>> {
            let size = self.get_size();
            if k > size {
                return None;
            }

            let mut slow = self.head.as_deref().unwrap();
            let mut fast = self.head.as_deref().unwrap();
            let mut count = 0;
            while count < k {
                fast = fast.next.as_deref().unwrap();
                count += 1;
            }

            while fast.next.is_some() {
                slow = slow.next.as_deref().unwrap();
                fast = fast.next.as_deref().unwrap();
            }

            return Some(slow);
        }

        pub fn insert_last(&mut self, data: T) {
            let mut current = &mut self.head;
            let new_node: Node<T> = Node::new_with(data).into();
            loop {
                match current {
                    None => {
                        self.head = Some(new_node.into());
                        break;
                    }
                    Some(ref mut node) if node.next.is_none() => {
                        node.next = new_node.into();
                        break;
                    }
                    Some(node) => current = &mut node.next,
                }
            }
        }

        pub fn remove_at(&mut self, index: usize) -> Option<Node<T>> {
            if index == 0 {
                if self.head.is_none() {
                    return None;
                }

                let new_head = self.head.as_deref_mut().unwrap().next.take();
                let old_head = self.head.take();
                self.head = new_head;
                return Some(*old_head.unwrap());
            }
            let mut count = 0;
            let mut current = &mut self.head;
            loop {
                match current {
                    None => return None,
                    Some(ref mut node) if count == index && node.next.is_none() => {
                        return Some(*current.take().unwrap());
                    }
                    Some(ref mut node) if count == index - 1 && node.next.is_some() => {
                        let mut node_to_remove = node.next.take();
                        let the_rest = node_to_remove.as_deref_mut().and_then(|n| n.next.take());
                        node.next = the_rest;
                        return Some(*node_to_remove.unwrap());
                    }
                    Some(_) if count == index - 1 => {
                        return None;
                    }
                    Some(node) => {
                        current = &mut node.next;
                        count += 1
                    }
                }
            }
        }

        pub fn insert_at(&mut self, index: usize, data: T) {
            if index == 0 || self.head.is_none() {
                return self.insert_first(data);
            }

            let mut count = 0;
            let mut current = &mut self.head;

            let mut node_to_insert = Node::new_with(data);
            loop {
                match current {
                    Some(ref mut node) if node.next.is_none() => {
                        node.next = node_to_insert.into();
                        return;
                    }
                    Some(ref mut node) if count == index - 1 => {
                        let the_rest = node.next.take();
                        node_to_insert.next = the_rest.into();
                        node.next = node_to_insert.into();
                        return;
                    }
                    Some(node) => {
                        count += 1;
                        current = &mut node.next;
                    }
                    None => {
                        return;
                    }
                }
            }
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a Node<T>;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                node
            })
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn can_create_node() {
            let mut n = Node::<i32>::new_with(5);
            let next = Node::<i32>::new_with(10);
            n.set_next(next.into());
            assert_eq!(n.data, 5);
            assert_eq!(n.next.unwrap().data, 10);
        }

        #[test]
        fn can_push_to_list() {
            let mut linked_list = LinkedList::<i32>::new();
            assert!(linked_list.get_first().is_none());
            linked_list.insert_first(10);
            assert_eq!(linked_list.get_first().as_ref().unwrap().data, 10);
            linked_list.insert_first(21);
            assert_eq!(linked_list.get_first().as_ref().unwrap().data, 21);
            assert_eq!(linked_list.get_size(), 2);
            assert_eq!(linked_list.remove_first().unwrap().data, 21);
            assert_eq!(linked_list.remove_first().unwrap().data, 10);
            assert_eq!(linked_list.remove_first(), None);
            assert_eq!(linked_list.get_size(), 0);

            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_size(), 5);
            linked_list.clear();
            assert_eq!(linked_list.get_size(), 0);
        }

        #[test]
        fn can_take_last() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_size(), 5);
            assert_eq!(linked_list.remove_last().unwrap().data, 100);
            assert_eq!(linked_list.get_size(), 4);
        }

        #[test]
        fn can_get_at() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_at(0).unwrap().data, 96);
            assert_eq!(linked_list.get_at(2).unwrap().data, 80);
            assert_eq!(linked_list.get_at(4).unwrap().data, 100);
            assert_eq!(linked_list.get_at(4).unwrap().data, 100);
        }

        #[test]
        fn insert_last() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_last(100);
            println!("linked_list.insert_last(100):{:?}", linked_list);
            linked_list.insert_last(1);
            println!("linked_list.insert_last(1):{:?}", linked_list);
            linked_list.insert_last(80);
            println!("linked_list.insert_last(80):{:?}", linked_list);
            linked_list.insert_last(4);
            println!("linked_list.insert_last(4):{:?}", linked_list);
            linked_list.insert_last(96);
            println!("linked_list.insert_last(96):{:?}", linked_list);
            assert_eq!(linked_list.get_at(0).unwrap().data, 100);
            assert_eq!(linked_list.get_at(2).unwrap().data, 80);
            assert_eq!(linked_list.get_at(4).unwrap().data, 96);
            assert_eq!(linked_list.get_at(4).unwrap().data, 96);
        }

        #[test]
        fn remove_at_inner() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_size(), 5);
            assert_eq!(linked_list.remove_at(2).unwrap().data, 80);
            assert_eq!(linked_list.get_size(), 4);
            assert_eq!(linked_list.get_at(1).unwrap().data, 4);
            assert_eq!(linked_list.get_at(2).unwrap().data, 1);
            assert_eq!(linked_list.get_at(3).unwrap().data, 100);
        }

        #[test]
        fn remove_at_past_last() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_size(), 5);
            assert!(linked_list.remove_at(7).is_none());
            assert!(linked_list.remove_at(6).is_none());
            assert!(linked_list.remove_at(5).is_none());
            assert_eq!(linked_list.get_size(), 5);
            assert_eq!(linked_list.get_at(4).unwrap().data, 100);

            assert_eq!(linked_list.remove_at(4).unwrap().data, 100);
            assert_eq!(linked_list.get_size(), 4);
            assert_eq!(linked_list.get_at(3).unwrap().data, 1);

            assert_eq!(linked_list.remove_at(0).unwrap().data, 96);
            assert_eq!(linked_list.get_size(), 3);
            assert_eq!(linked_list.get_at(0).unwrap().data, 4);
        }

        #[test]
        fn insert_at() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_size(), 5);

            linked_list.insert_at(2, 21);
            assert_eq!(linked_list.get_at(1).unwrap().data, 4);
            assert_eq!(linked_list.get_at(2).unwrap().data, 21);
            assert_eq!(linked_list.get_at(3).unwrap().data, 80);
            assert_eq!(linked_list.get_size(), 6);

            linked_list.insert_at(0, 123);
            assert_eq!(linked_list.get_first().unwrap().data, 123);
            assert_eq!(linked_list.get_at(1).unwrap().data, 96);
            assert_eq!(linked_list.get_size(), 7);

            linked_list.insert_at(99999, 99);
            assert_eq!(linked_list.get_size(), 8);
            assert_eq!(linked_list.get_last().unwrap().data, 99);
            assert_eq!(linked_list.get_at(6).unwrap().data, 100);
        }

        #[test]
        fn get_middle() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_size(), 5);
            assert_eq!(linked_list.get_middle().unwrap().data, 80);
            linked_list.insert_first(0);
            assert_eq!(linked_list.get_size(), 6);
            assert_eq!(linked_list.get_middle().unwrap().data, 4);
        }

        #[test]
        fn get_from_end() {
            let mut linked_list = LinkedList::<i32>::new();
            linked_list.insert_first(100);
            linked_list.insert_first(1);
            linked_list.insert_first(80);
            linked_list.insert_first(4);
            linked_list.insert_first(96);
            assert_eq!(linked_list.get_from_last(0).unwrap().data, 100);
            assert_eq!(linked_list.get_from_last(1).unwrap().data, 1);
            assert_eq!(linked_list.get_from_last(2).unwrap().data, 80);
        }
    }
}
