from typing import List
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution0114:
    def flatten(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        def dfs(node: TreeNode) -> (TreeNode, TreeNode):
            l_t, l_b = None, None
            if node.left is not None:
                (l_t, l_b) = dfs(node.left)
            r_t, r_b = None, None
            if node.right is not None:
                (r_t, r_b) = dfs(node.right)
            node.left = None
            res_t, res_b = None, None
            if r_t is not None:
                if l_b is not None:
                    l_b.right = r_t
                    node.right = l_t
                else:
                    node.right = r_t
                res_b = r_b
            else:
                if l_b is not None:
                    node.right = l_t
                    res_b = l_b
                else:
                    res_b = node
                    node.right = None
            return (node, res_b)
            
        if root is None:
            return None
        dfs(root)
        return root