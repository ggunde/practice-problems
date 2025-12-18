from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        self.inorder_output: List[int] = []
        self.inorder(root)
        return self.inorder_output

    def inorder(self, node: Optional[TreeNode]):
        if node is None:
            return

        self.inorder(node.left)
        self.inorder_output.append(node.val)
        self.inorder(node.right)

    def inorderTraversalIterative(self, root: Optional[TreeNode]) -> List[int]:
        stack: List[TreeNode] = []
        inorder_output: List[int] = []
        current = root
        while current is not None:
            stack.append(current)
            current = current.left
        while len(stack) != 0:
            current = stack.pop()
            inorder_output.append(current.val)

            if current.right:
                current = current.right
                while current is not None:
                    stack.append(current)
                    current = current.left

        return inorder_output
