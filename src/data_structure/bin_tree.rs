use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<BinTreeNode>>>;

struct BinTreeNode {
    parent: Link,
    value: usize,
    left: Option<BinTree>,
    right: Option<BinTree>,
}

impl BinTreeNode {
    fn new(value: usize) -> Self {
        Self {
            parent: None,
            value: value,
            left: None,
            right: None,
        }
    } 
}

struct BinTree {
    root: Link,
    // height: usize,
}

impl BinTree {
    fn new(value: usize) -> Self {
        let node = BinTreeNode::new(value);

        Self {
            root: Some(Rc::new(RefCell::new(node))),
        }
    }

    fn read_root() -> usize {
        0
    }

    fn read_left() -> usize {
        0
    }

    fn read_right() -> usize {
        0
    }

    fn insert_node_left(&mut self, value: usize) -> Result<(), ()> {
        if self.root.is_none() { return Err(()) }
        let mut root = self.root.as_ref().unwrap().borrow_mut();
        if root.left.is_some() { return Err(()) }

        let node = BinTreeNode::new(value);
        let link = Some(Rc::new(RefCell::new(node)));
        root.left = Some(BinTree{ root: link });

        Ok(())
    }

    fn insert_node_right(&mut self, value: usize) -> Result<(), ()> {
        if self.root.is_none() { return Err(()) }
        let mut root = self.root.as_ref().unwrap().borrow_mut();
        if root.right.is_some() { return Err(()) }

        let node = BinTreeNode::new(value);
        let link = Some(Rc::new(RefCell::new(node)));
        root.right = Some(BinTree{ root: link });
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert() {
        let tree = super::BinTree::new(0);
    
        assert_eq!(tree.root.as_ref().unwrap().borrow().value, 0);
    }
}