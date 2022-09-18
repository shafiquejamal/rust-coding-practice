#[derive(Debug, Default)]
pub struct Queue<T> {
    list: Vec<T>,
}

impl<T: Default> Queue<T> {
    pub fn new() -> Self {
        Queue::default()
    }

    pub fn add(&mut self, elem: T) {
        self.list.push(elem)
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.list.is_empty() {
            None
        } else {
            self.list.get(0)
        }
    }
}

pub fn weave<T: Default + Clone>(q1: Queue<T>, q2: Queue<T>) -> Queue<T> {
    let mut q = Queue::<T>::new();
    while q1.peek().is_some() || q2.peek().is_some() {
        if q1.peek().is_some() {
            let elem = q1.peek().unwrap().clone();
            q.add(elem)
        }

        if q2.peek().is_some() {
            let elem = q2.peek().unwrap().clone();
            q.add(elem)
        }
    }
    q
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_add_and_remove() {
        let mut q = Queue::<u32>::new();
        assert_eq!(q.peek(), None);
        q.add(1);
        assert_eq!(q.peek(), Some(&1));
        q.add(2);
        assert_eq!(q.peek(), Some(&1));
        q.add(3);
        assert_eq!(q.peek(), Some(&1));
        assert_eq!(q.remove(), Some(1));
        assert_eq!(q.peek(), Some(&2));
        assert_eq!(q.remove(), Some(2));
        assert_eq!(q.peek(), Some(&3));
        assert_eq!(q.remove(), Some(3));
        assert_eq!(q.peek(), None);
        assert_eq!(q.remove(), None);
    }

    fn can_weave() {
        let mut q1 = Queue::<u32>::new();
        q1.add(1);
        q1.add(3);
        q1.add(5);
        q1.add(7);
        q1.add(9);
        let mut q2 = Queue::<u32>::new();
        q2.add(2);
        q2.add(4);
        let mut q = weave(q1, q2);
        assert_eq!(q.remove(), Some(1));
        assert_eq!(q.remove(), Some(2));
        assert_eq!(q.remove(), Some(3));
        assert_eq!(q.remove(), Some(4));
        assert_eq!(q.remove(), Some(5));
        assert_eq!(q.remove(), Some(7));
        assert_eq!(q.remove(), Some(9));
    }
}
