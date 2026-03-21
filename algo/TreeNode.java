import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

class leetcode0652 {
    public static class Pairp {
        public boolean matched = false;
        public TreeNode node;

        public Pairp(boolean matched, TreeNode node) {
            this.matched = matched;
            this.node = node;
        }
    }

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
    
        TreeNode() {
        }
    
        TreeNode(int val) {
            this.val = val;
        }
    
        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    public List<TreeNode> findDuplicateSubtrees(TreeNode root) {
        HashMap<Integer, ArrayList<Pairp>> map = new HashMap<>();
        ArrayList<TreeNode> res = new ArrayList<>();
        dfs(root, res, map);
        return res;
    }

    private void dfs(
            TreeNode node,
            ArrayList<TreeNode> res,
            HashMap<Integer, ArrayList<Pairp>> map) {
        if (node == null) {
            return;
        }
        ArrayList<Pairp> cands = map.getOrDefault(node.val, new ArrayList<>());
        boolean to_add_to_map = true;
        for (int i = 0; i < cands.size(); i++) {
            Pairp pair = cands.get(i);
            if (equal(node, pair.node)) {
                if (!pair.matched) {
                    pair.matched = true;
                    res.add(node);
                }
                to_add_to_map = false;
                break;
            }
        }
        if (to_add_to_map) {
            cands.add(new Pairp(false, node));
            map.put(node.val, cands);
        }
        dfs(node.left, res, map);
        dfs(node.right, res, map);
    }

    private boolean equal(TreeNode node1, TreeNode node2) {
        if (node1 == null && node2 == null) {
            return true;
        }
        if ((node1 != null && node2 == null)
                || (node1 == null && node2 != null)) {
            return false;
        }
        if (node1.val != node2.val) {
            return false;
        }
        return equal(node1.left, node2.left)
                && equal(node1.right, node2.right);
    }

    public static void main(String[] args) {
        
    }
}