# LC160 相交链表（Rust 实现讲解）

## 链表定义
- 节点用 `Rc<ListNode>` 共享所有权，便于两条链表共享尾部而无需可变引用。
- 结构体：`ListNode { val: i32, next: Option<Rc<ListNode>> }`，构造函数 `ListNode::new(val, next)` 返回 `Rc<ListNode>`。

## 构建与打印工具
- `build_list(vals, tail)`：从后往前把 `vals` 串成链表，尾部接到 `tail`，返回 `Option<Rc<ListNode>>`。
- `build_intersected_lists(prefix_a, prefix_b, common)`：先建公共尾 `common_head`，再把 `prefix_a`、`prefix_b` 接上，生成两条带交点的链表，并返回 `(head_a, head_b, common_head)`。
- `print_list`：克隆指针遍历，收集数值后打印，便于快速查看链表。

## 指针比较
- `ptr_eq_option` 通过 `Rc::ptr_eq` 判断两个 `Option<Rc<_>>` 是否指向同一物理节点；`None` 与 `None` 视作相等。

## 核心算法：双指针
- 函数 `get_intersection_node(head_a, head_b)`：
  - 两个指针 `pa`、`pb` 分别从 `head_a`、`head_b` 出发。
  - 每步向后走；走到尾后切换到另一条链表的头。最多 `m+n` 步，两指针会在交点相遇，或同时到 `None`。
  - 伪代码：
    ```rust
    while !ptr_eq_option(&pa, &pb) {
        pa = pa.as_ref().map(|n| n.next.clone()).unwrap_or_else(|| head_b.clone());
        pb = pb.as_ref().map(|n| n.next.clone()).unwrap_or_else(|| head_a.clone());
    }
    // pa 即交点或 None
    ```

## 示例与运行
- 示例 1（有交点）：A = `[4,1] + [8,4,5]`，B = `[5,6,1] + 同尾`，返回值节点 `val=8`，且 `ptr_eq_option(inter, common_head)==true`。
- 示例 2（无交点）：A = `[2,6,4]`，B = `[1,5]`，两指针最终都为 `None`。
- 运行示例：在仓库根目录执行 `rust-script hot100/lc160.rs`。
