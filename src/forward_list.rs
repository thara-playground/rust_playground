use std::ops::Index;

#[derive(Debug)]
enum ForwardList<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: ForwardList<T>,
}

use self::ForwardList::*;

impl<T> ForwardList<T> {
    fn size(&self) -> usize {
        let mut current = self;
        let mut n = 0;
        while let NonEmpty(node) = current {
            n += 1;
            current = &(*&node).next
        }
        n
    }
}

impl<T> Index<usize> for ForwardList<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let mut current = self;
        let mut n = 0;
        while let NonEmpty(node) = current {
            if n == i {
                return &(*&node).value;
            }
            n += 1;
            current = &(*&node).next
        }
        panic!("Out of range");
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::*;
    use std::panic;

    #[test]
    fn test_size() {
        let list = NonEmpty(Box::new(Node {
            value: "tokyo",
            next: NonEmpty(Box::new(Node {
                value: "nagoya",
                next: NonEmpty(Box::new(Node {
                    value: "osaka",
                    next: Empty,
                })),
            })),
        }));
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn test_index() {
        let list = NonEmpty(Box::new(Node {
            value: "tokyo",
            next: NonEmpty(Box::new(Node {
                value: "nagoya",
                next: NonEmpty(Box::new(Node {
                    value: "osaka",
                    next: Empty,
                })),
            })),
        }));
        assert_eq!(list[0], "tokyo");
        assert_eq!(list[1], "nagoya");
        assert_eq!(list[2], "osaka");

        let result = panic::catch_unwind(|| {
            let list: ForwardList<&str> = Empty;
            assert_eq!(list[0], "tokyo");
        });
        assert!(result.is_err());

        let result = panic::catch_unwind(|| {
            assert_eq!(list[3], "osaka");
        });
        assert!(result.is_err());
    }
}
