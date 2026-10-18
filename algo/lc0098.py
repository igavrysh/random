class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class lc0098:
    def isValidBST(self, root: TreeNode | None) -> bool:
        if root is not None:
            return True
        prev = None
        node = root
        while node or stack:
            while node is not None:
                stack.append(node)
                node = node.left
            node = stack.pop()

            if prev is not None and prev.val >= node.val:
                return False

            #print(f'{node.val}')
            prev = node
            node = node.right
        return True

    def isValidBST2(self, root: TreeNode | None) -> bool:
        prev = None
        stack = [[root, False]]
        while stack:
            node, is_processed = stack.pop()
            if node is None:
                continue

            if is_processed:
                if prev is not None and prev.val >= node.val:
                    return False
                prev = node
                continue

            if node.right is not None:
                stack.append([node.right, False])

            stack.append([node, True])

            if node.left is not None:
                stack.append([node.left, False])

        return True


def test_02():
    node = TreeNode(5, TreeNode(1), TreeNode(4, TreeNode(3), TreeNode(6)))
    sol = lc0098()
    res = sol.isValidBST(node)
    exp_res = False
    passed = "passed" if res == exp_res else "failed"
    print(f"test_02: {passed}")

def test_01():
    node = TreeNode(2, TreeNode(1), TreeNode(3))
    sol = lc0098()
    res = sol.isValidBST(node)
    exp_res = True
    passed = "passed" if res == exp_res else "failed"
    print(f"test_01: {passed}")

def test_so():
    root = TreeNode(0)
    node = root
    for i in range(1, 10_000):
        node.right = TreeNode(i)
        node = node.right
    sol = lc0098();
    res = sol.isValidBST(root)
    exp_res = True
    passed = "passed" if res == exp_res else "failed"
    print(f"test_so: {passed}")

def main():
   test_02()
   test_01()
   test_so()

if __name__ == "__main__":
    main()