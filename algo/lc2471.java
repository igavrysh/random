import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

class lc2471 {
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode() {}
        TreeNode(int val) { this.val = val; }
        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }
    public int minimumOperations(TreeNode root) {
        Queue<TreeNode> q = new LinkedList<>();
        if (root != null) {
            q.offer(root);
        }
        int res = 0;
        while (!q.isEmpty()) {
            int l = q.size();
            ArrayList<TreeNode> lev = new ArrayList<>();
            ArrayList<Integer> a = new ArrayList<>();
            for (int i = 0; i < l; i++) {
                TreeNode node = q.poll();
                lev.add(node);
                a.add(node.val);
                if (node.left != null) {
                    q.offer(node.left);
                }
                if (node.right != null) {
                    q.offer(node.right);
                }
            }
            int n = a.size();
            ArrayList<Integer> sorted_a = new ArrayList<>(List.copyOf(a));
            Collections.sort(sorted_a, Integer::compare);
            HashMap<Integer, Integer> pos = new HashMap<>();
            for (int j = 0; j < n; j++) {
                pos.put(a.get(j), j);
            }
            int delta = 0;
            for (int j = 0; j < n; j++) {
                if (!a.get(j).equals(sorted_a.get(j))) {
                    int source_indx = pos.get(sorted_a.get(j));
                    int val = a.get(j);
                    pos.put(sorted_a.get(j), j);
                    pos.put(val, source_indx);
                    a.set(j, sorted_a.get(j));
                    a.set(source_indx, val);
                    delta++;
                }
            }
            res += delta;
        }
        return res;
    }

    public static void main(String[] args) {
        test1();
    }

    public static void test1() {
        Integer[] input = {
            332,463,103,417,150,409,41,135,129,117,474,263,null,328,456,347,167,
            383,null,null,422,493,489,275,72,null,null,425,89,null,null,162,18,
            null,null,null,null,363,290,106,260,468,null,null,null,432,null,323,
            null,null,null,null,null,null,36,null,null,302,190,null,280,null,
            null,null,null,488,null,null,null,null,446,null,null,null,null,null,75};
        TreeNode root = buildTree(input);
        lc2471 sol = new lc2471();
        int exp_output = 24;
        int output = sol.minimumOperations(root);
        boolean passed = output == exp_output;
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }

    private static TreeNode buildTree(Integer[] input) {
        Queue<TreeNode> q = new LinkedList<>();
        if (input.length == 0) {
            return null;
        }
        TreeNode root = new TreeNode(input[0]);
        q.offer(root);
        int idx = 1;
        while (!q.isEmpty()) {
            TreeNode node = q.poll();
            if (idx < input.length) {
                node.left = input[idx] != null ? new TreeNode(input[idx]) : null;
                idx++;
                if (node.left != null) {
                    q.offer(node.left);
                }
            }
            if (idx < input.length) {
                node.right = input[idx] != null ? new TreeNode(input[idx]) : null;
                idx++;
                if (node.right != null) {
                    q.offer(node.right);
                }
            }
        }
        return root;
    }
}

/*
2471. Minimum Number of Operations to Sort a Binary Tree by Level

Medium

You are given the root of a binary tree with unique values.

In one operation, you can choose any two nodes at the same level and swap their values.

Return the minimum number of operations needed to make the values at each level sorted in a strictly increasing order.

The level of a node is the number of edges along the path between it and the root node.

Example 1:
    1
    /\
   4  3
  /\  |\
 7 8  8 5
     /  /
    9  10

Input: root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
Output: 3
Explanation:
- Swap 4 and 3. The 2nd level becomes [3,4].
- Swap 7 and 5. The 3rd level becomes [5,6,8,7].
- Swap 8 and 7. The 3rd level becomes [5,6,7,8].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.

Example 2:
  1
 / \
 3  2
/\  /\
7 6 5 4

Input: root = [1,3,2,7,6,5,4]
Output: 3
Explanation:
- Swap 3 and 2. The 2nd level becomes [2,3].
- Swap 7 and 4. The 3rd level becomes [4,6,5,7].
- Swap 6 and 5. The 3rd level becomes [4,5,6,7].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.

Example 3:
   1
  / \
 2   3
/\  /
4 5 6
Input: root = [1,2,3,4,5,6]
Output: 0
Explanation: Each level is already sorted in increasing order so return 0.

Constraints:

The number of nodes in the tree is in the range [1, 10^5].
1 <= Node.val <= 10^5
All the values of the tree are unique.
 */