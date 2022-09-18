#[derive(Debug, Default)]
pub struct Stack<T> {
    v: Vec<T>,
}

impl<T: Default> Stack<T> {
    pub fn new() -> Self {
        Stack::<T>::default()
    }

    pub fn push(&mut self, elem: T) {
        self.v.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.v.is_empty() {
            None
        } else {
            Some(self.v.remove(self.v.len() - 1))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.v.is_empty() {
            None
        } else {
            self.v.get(self.v.len() - 1)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_add_and_remove() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.pop(), None);
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
    }
}
