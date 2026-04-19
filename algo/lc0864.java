import java.util.List;
import java.util.ArrayList;

public class lc0864 {
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int x) { val = x; }
        TreeNode(int val, TreeNode left, TreeNode right) { 
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }
    
    public List<Integer> distanceK(TreeNode root, TreeNode target, int k) {
        List<Integer> acc = new ArrayList<>();
        dfs(root, false, k, target.val, k, acc);
        return acc;
    }

    private int dfs(TreeNode node, boolean found, int curr_k, int target, int k, List<Integer> acc) {
        if (node == null) {
            return -1;
        }
        if (found && curr_k == 0) {
            acc.add(node.val);
            return -1;
        }
         if (node.val == target || found) {
            found = true;
            if (node.val == target && k == 0) {
                acc.add(node.val);
            } else {
                dfs(node.left, true, curr_k-1, target, k, acc);
                dfs(node.right, true, curr_k-1, target, k, acc);
            }
            return k-1;
        }

        int res = -1;
        boolean found_left = false;
        boolean found_right = false;
        if (!found) {
            res = dfs(node.left, false, k, target, k, acc);
            if (res == -1) {
                res = dfs(node.right, false, k, target, k, acc);
                if (res != -1) {
                    found = true;
                    found_right = true;
                }
            } else {
                found = true;
                found_left = true;
            }
        }

        if (res == 0) {
            acc.add(node.val);
        } else {
            if (found_left) {
                dfs(node.right, true, res-1, target, k, acc);
            } else if (found_right) {
                dfs(node.left, true, res-1, target, k, acc);
            }
        }       
        return found ? res-1 : -1;
    }

    public static void test1() {
        TreeNode root = new TreeNode(1, new TreeNode(1, new TreeNode(3), new TreeNode(2)), null);
        lc0864 sol = new lc0864();
        List<Integer> output = sol.distanceK(root, new TreeNode(2), 1);
        System.out.println("output: " + output);
    }

    public static void test2() {
        TreeNode root = new TreeNode(0, new TreeNode(2), new TreeNode(1, new TreeNode(3), null));
        lc0864 sol = new lc0864();
        List<Integer> output = sol.distanceK(root, new TreeNode(3), 3);
        System.out.println("output: " + output);
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(0, new TreeNode(1, null, new TreeNode(2, null, new TreeNode(3, null, new TreeNode(4)))), null);
        lc0864 sol = new lc0864();
        List<Integer> output = sol.distanceK(root, new TreeNode(3), 0);
        System.out.println("output: " + output);
    }
}