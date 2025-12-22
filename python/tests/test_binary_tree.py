from practice_problems.binary_tree import Solution, TreeNode


def test_binary_tree_inorder_traversal():
    root = TreeNode(1, TreeNode(2), TreeNode(3))
    assert Solution().inorderTraversal(root) == [2, 1, 3], (
        f"Expected [2, 1, 3], but got {Solution().inorderTraversal(root)}"
    )

    root = TreeNode(1, TreeNode(2), TreeNode(3, TreeNode(4), TreeNode(5)))
    assert Solution().inorderTraversal(root) == [2, 1, 4, 3, 5], (
        f"Expected [2, 1, 4, 3, 5], but got {Solution().inorderTraversal(root)}"
    )


def test_binary_tree_inorder_traversal_iterative():
    root = TreeNode(1, TreeNode(2), TreeNode(3))
    assert Solution().inorderTraversalIterative(root) == [2, 1, 3], (
        f"Expected [2, 1, 3], but got {Solution().inorderTraversalIterative(root)}"
    )

    root = TreeNode(1, TreeNode(2), TreeNode(3, TreeNode(4), TreeNode(5)))
    assert Solution().inorderTraversalIterative(root) == [2, 1, 4, 3, 5], (
        f"Expected [2, 1, 4, 3, 5], but got {Solution().inorderTraversalIterative(root)}"
    )
