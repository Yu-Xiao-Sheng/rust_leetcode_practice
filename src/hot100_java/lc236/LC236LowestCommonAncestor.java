package hot100_java.lc236;

import java.util.HashMap;
import java.util.Map;

/**
 * 236. 二叉树的最近公共祖先
 * 递归后序查找，时间 O(n)，空间 O(h)。
 */
public class LC236LowestCommonAncestor {

    private static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) { this.val = val; }
    }

    public static TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        if (root == null || root == p || root == q) {
            return root;
        }
        TreeNode left = lowestCommonAncestor(root.left, p, q);
        TreeNode right = lowestCommonAncestor(root.right, p, q);
        if (left != null && right != null) {
            return root;
        }
        return left != null ? left : right;
    }

    private static TreeNode buildTree(Integer[] data) {
        if (data.length == 0 || data[0] == null) {
            extraMap = new HashMap<>();
            return null;
        }
        TreeNode[] nodes = new TreeNode[data.length];
        Map<Integer, TreeNode> map = new HashMap<>();
        for (int i = 0; i < data.length; i++) {
            if (data[i] != null) {
                nodes[i] = new TreeNode(data[i]);
                map.put(data[i], nodes[i]);
            }
        }
        for (int i = 0; i < data.length; i++) {
            TreeNode node = nodes[i];
            if (node == null) continue;
            int li = 2 * i + 1, ri = 2 * i + 2;
            if (li < data.length) node.left = nodes[li];
            if (ri < data.length) node.right = nodes[ri];
        }
        extraMap = map; // 保存值->节点映射，示例查找使用
        return nodes[0];
    }

    // 仅示例使用，避免重复创建 Map
    private static Map<Integer, TreeNode> extraMap = new HashMap<>();

    private static TreeNode findByVal(int v) {
        return extraMap.get(v);
    }

    private static void runCase(Integer[] data, int pVal, int qVal, String label) {
        TreeNode root = buildTree(data);
        TreeNode p = findByVal(pVal);
        TreeNode q = findByVal(qVal);
        System.out.println("案例 " + label + ": p=" + pVal + ", q=" + qVal);
        if (p == null || q == null) {
            System.out.println("输入节点不存在");
            System.out.println();
            return;
        }
        TreeNode lca = lowestCommonAncestor(root, p, q);
        System.out.println("最近公共祖先: " + (lca == null ? "null" : lca.val));
        System.out.println();
    }

    public static void main(String[] args) {
        Integer[] sample1 = {3, 5, 1, 6, 2, 0, 8, null, null, 7, 4};
        runCase(sample1, 5, 1, "一");
        runCase(sample1, 5, 4, "二");

        Integer[] sample2 = {1, 2};
        runCase(sample2, 1, 2, "三");
    }
}
