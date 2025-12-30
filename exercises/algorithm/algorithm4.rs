/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(root_node) => {
                // 根节点存在，调用TreeNode的插入方法递归处理
                root_node.insert(value);
            }
            None => {
                // 根节点不存在，初始化根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(value))
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 待插入值小于当前节点值：处理左子树
            Ordering::Less => {
                match &mut self.left {
                    Some(left_node) => left_node.insert(value),  // 左子树存在，递归插入
                    None => self.left = Some(Box::new(TreeNode::new(value))),  // 左子树为空，创建新节点
                }
            }
            // 待插入值大于当前节点值：处理右子树
            Ordering::Greater => {
                match &mut self.right {
                    Some(right_node) => right_node.insert(value),  // 右子树存在，递归插入
                    None => self.right = Some(Box::new(TreeNode::new(value))),  // 右子树为空，创建新节点
                }
            }
            // 待插入值等于当前节点值：BST通常不允许重复值，直接返回
            Ordering::Equal => return,
        }
    }

    // Search for a value in the subtree rooted at this node
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),
            Ordering::Equal => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 测试空树搜索
        assert_eq!(bst.search(1), false);

        // 插入一些值
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 测试已插入值的搜索
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 测试未插入值的搜索
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复值
        bst.insert(1);
        bst.insert(1);

        // 搜索应该返回true
        assert_eq!(bst.search(1), true);

        // 检查树结构，确保没有创建重复节点
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
