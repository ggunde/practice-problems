// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder(root, &mut result);
        result
    }

    fn inorder(node: Option<Box<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::inorder(node.left, result);
            result.push(node.val);
            Self::inorder(node.right, result);
        }
    }

    pub fn inorder_traversal_iterative(root: Option<Box<TreeNode>>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut stack: Vec<TreeNode> = Vec::new();
        let mut current = root;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = Some(Box::new(TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
        }));

        let expected = vec![2, 1, 3];
        assert_eq!(Solution::inorder_traversal(input), expected);
    }
}
