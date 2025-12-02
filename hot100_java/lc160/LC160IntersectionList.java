/**
 * 160. 相交链表
 * 双指针，时间 O(m+n)，空间 O(1)。
 */
public class LC160IntersectionList {

    private static class ListNode {
        int val;
        ListNode next;
        ListNode(int val) {
            this.val = val;
        }
    }

    public static ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        ListNode pa = headA, pb = headB;
        while (pa != pb) {
            pa = (pa == null) ? headB : pa.next;
            pb = (pb == null) ? headA : pb.next;
        }
        return pa;
    }

    private static ListNode buildList(int[] vals, ListNode tail) {
        ListNode head = tail;
        for (int i = vals.length - 1; i >= 0; i--) {
            ListNode node = new ListNode(vals[i]);
            node.next = head;
            head = node;
        }
        return head;
    }

    private static ListNode[] buildIntersectedLists(int[] prefixA, int[] prefixB, int[] common) {
        ListNode commonHead = buildList(common, null);
        ListNode headA = buildList(prefixA, commonHead);
        ListNode headB = buildList(prefixB, commonHead);
        return new ListNode[]{headA, headB, commonHead};
    }

    private static void printList(ListNode head) {
        StringBuilder sb = new StringBuilder("[");
        ListNode cur = head;
        while (cur != null) {
            sb.append(cur.val);
            cur = cur.next;
            if (cur != null) {
                sb.append(", ");
            }
        }
        sb.append("]");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        // 示例一：存在交点
        ListNode[] lists1 = buildIntersectedLists(new int[]{4, 1}, new int[]{5, 6, 1}, new int[]{8, 4, 5});
        ListNode headA1 = lists1[0], headB1 = lists1[1], common = lists1[2];
        printList(headA1);
        printList(headB1);
        ListNode inter1 = getIntersectionNode(headA1, headB1);
        System.out.println("交点值: " + (inter1 == null ? "null" : inter1.val));
        System.out.println("与公共部分首节点相同引用? " + (inter1 == common));
        System.out.println();

        // 示例二：无交点
        ListNode[] lists2 = buildIntersectedLists(new int[]{2, 6, 4}, new int[]{1, 5}, new int[]{});
        ListNode inter2 = getIntersectionNode(lists2[0], lists2[1]);
        System.out.println("交点值: " + (inter2 == null ? "null" : inter2.val));
    }
}
