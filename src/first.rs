type Link<T> = Option<Box<Node<T>>>; // Box<> is like unique_ptr<>

pub struct List<T> {
    head: Link<T>, // Optional pointer on the stack, actual nodes on the heap
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

// Avoid recursion in default drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        while let Some(mut node) = next {
            // Move `node.next` to local `next`, replacing with `None`
            next = node.next.take();
            // `node` goes out of scope and is dropped
            // Since `node.next` is now `None`, no recursion happens
        }
    }
}

pub struct ListIntoIter<T>(List<T>);

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
    }
}

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter {next: self.head.as_ref().map(|h| { h.as_ref() })}
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| node.as_ref());
            &node.elem
        })
    }
}

pub struct ListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = ListIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterMut {next: self.head.as_mut().map(|h| { h.as_mut() })}
    }
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| node.as_mut());
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_and_pop() {
        let mut l = List::new(); // Type parameter inferred below
        assert_eq!(None, l.pop());

        l.push(1);
//        l.push('a'); // Expected `i32` found `char`
        l.push(2);
        l.push(3);
        assert_eq!(Some(3), l.pop());
        assert_eq!(Some(2), l.pop());

        l.push(4);
        l.push(5);
        assert_eq!(Some(5), l.pop());
        assert_eq!(Some(4), l.pop());
        assert_eq!(Some(1), l.pop());
        assert_eq!(None, l.pop());
    }

    #[test]
    fn peek() {
        let mut l = List::new();

        l.push(1);
        assert_eq!(Some(&1), l.peek());
        l.peek_mut().map(|p| {
            *p = 42;
        });
        assert_eq!(Some(&42), l.peek());
        assert_eq!(Some(42), l.pop());
    }

    #[test]
    fn into_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut i = 3;
        for e in l {
            assert_eq!(i, e);
            i -= 1;
        }
//        assert_eq!(Some(3), l.pop()); // l was moved into the loop
    }

    #[test]
    fn into_iter_ref() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut i = 3;
        for e in &l {
            assert_eq!(&i, e);
//            *e = 42; // No mutation: & is immutable reference
            i -= 1;
        }
        // Can continue using l since it was borrowed to the loop, not moved into it
        assert_eq!(Some(3), l.pop());
        assert_eq!(Some(2), l.pop());
        assert_eq!(Some(1), l.pop());
        assert_eq!(None, l.pop());
    }

    #[test]
    fn into_iter_mut() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut i = 3;
        for e in &mut l {
            assert_eq!(&i, e);
            *e = i * 10; // Mutation, yay...
            i -= 1;
        }
        assert_eq!(Some(30), l.pop());
        assert_eq!(Some(20), l.pop());
        assert_eq!(Some(10), l.pop());
        assert_eq!(None, l.pop());
    }
}
