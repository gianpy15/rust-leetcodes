use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn add_left(mut self, val: i32) -> Rc<RefCell<TreeNode>> {
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        self.left = Some(Rc::clone(&node));
        node
    }

    pub fn add_right(mut self, val: i32) -> Rc<RefCell<TreeNode>> {
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        self.right = Some(Rc::clone(&node));
        node
    }
}

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        root.map_or(false, |node| {
            let borrowed_node = &*node.borrow();
            match borrowed_node {
                &TreeNode {
                    val,
                    left: None,
                    right: None,
                } => val == target_sum,
                &TreeNode {
                    val,
                    ref left,
                    ref right,
                } => {
                    Self::has_path_sum(left.clone(), target_sum - val)
                        || Self::has_path_sum(right.clone(), target_sum - val)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree_node(mut values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let tree = Rc::new(RefCell::new(TreeNode::new(values.remove(0).unwrap())));
        let mut bfs = vec![Rc::clone(&tree)];

        while !values.is_empty() {
            let left_value = values.remove(0);
            let right_value = values.remove(0);
            let parent = bfs.remove(0);

            let left_node = match left_value {
                Some(number) => {
                    let node = Rc::new(RefCell::new(TreeNode::new(number)));
                    bfs.push(Rc::clone(&node));
                    Some(Rc::clone(&node))
                }
                None => None,
            };

            let right_node = match right_value {
                Some(number) => {
                    let node = Rc::new(RefCell::new(TreeNode::new(number)));
                    bfs.push(Rc::clone(&node));
                    Some(Rc::clone(&node))
                }
                None => None,
            };

            parent.clone().borrow_mut().left = left_node;
            parent.clone().borrow_mut().right = right_node;
        }

        Some(Rc::clone(&tree))
    }

    fn values() -> Vec<Option<i32>> {
        vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]
    }

    #[test]
    fn test_has_path_sum_22() {
        let tree = create_tree_node(values());
        assert!(Solution::has_path_sum(tree, 22));
    }

    #[test]
    fn test_has_path_sum_26() {
        let tree = create_tree_node(values());
        assert!(Solution::has_path_sum(tree, 26));
    }

    #[test]
    fn test_not_has_path_sum_10() {
        let tree = create_tree_node(values());
        assert!(!Solution::has_path_sum(tree, 10));
    }
}
