type Link = Option<Box<BiTreeNode>>;

struct BiTreeNode {
    value: usize,
    left: Link,
    right: Link,
    // parent: &,
}

struct BiTree {
    root: Link,
}

impl BiTree {
    fn zig(&mut self) {
        let mut right = self.root.take().unwrap();
        let mut left = right.left.take().unwrap();
        let mut center = left.right.take().unwrap();

        right.left = Some(center);
        left.right = Some(right);
        self.root = Some(left);
    }

    fn zag(&mut self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zig_and_zag_0() {
        let mut left = BiTreeNode {
            value: 0,
            left: None,
            right: None,
        };
        let center = BiTreeNode {
            value: 1,
            left: None,
            right: None,
        };
        let mut right = BiTreeNode {
            value: 2,
            left: None,
            right: None,
        };
        
        left.right = Some(Box::new(center));
        right.left = Some(Box::new(left));

        let mut tree = BiTree {
            root: Some(Box::new(right)),
        };

        tree.zig();

        let root = tree.root.unwrap();
        let node1 = root.right.unwrap();
        let node2 = node1.left.unwrap();
        assert_eq!(root.value, 0);
        assert_eq!(node2.value, 1);
        assert_eq!(node1.value, 2);
    }
}