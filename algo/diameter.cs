public class Solution
{
    public int DiameterOfBinaryTree(TreeNode root)
    {
        return GetLengthOfTheDiameterDFS(root).diameter;
    }
    
    public (int diameter, int pathToNode) GetLengthOfTheDiameterDFS(TreeNode root)
    {
        if (root == null) return (0, -1);

        var left = GetLengthOfTheDiameterDFS(root.left);
        var right = GetLengthOfTheDiameterDFS(root.right);

        var leftAndRightDiameterMax = Math.Max(left.diameter, right.diameter);
        var curDiameter = 2 + left.pathToNode + right.pathToNode;
        var maxDiameter = Math.Max(leftAndRightDiameterMax, curDiameter);
        return (maxDiameter, 1 + Math.Max(left.pathToNode, right.pathToNode));
    }

}