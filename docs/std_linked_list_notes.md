# Rust 标准库 LinkedList 速览

## 实现原理
- 双向链表，每个节点在堆上，含前驱/后继指针和元素值。
- `LinkedList<T>` 内部保存 `len`、`head`、`tail`（`Option<NonNull<Node<T>>>`），靠 `unsafe` 维护指针保证性能与安全 API。
- 头尾插入/删除为 O(1)，中间位置的遍历/插入/删除为 O(n)。
- 节点位置稳定：在链表上插删其他节点不会使已有节点的地址失效，适合需要稳定迭代器的场景。

## 基本用法
```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();
list.push_back(1);
list.push_front(0);
list.push_back(2);

assert_eq!(list.front(), Some(&0));
assert_eq!(list.back(), Some(&2));

assert_eq!(list.pop_front(), Some(0));
assert_eq!(list.pop_back(), Some(2));

for x in list.iter() {
    println!("{x}");
}

// 合并：把 other 移动到当前表尾，other 变空
let mut a: LinkedList<_> = [1,2].into_iter().collect();
let mut b: LinkedList<_> = [3,4].into_iter().collect();
a.append(&mut b); // a: 1,2,3,4  b: empty
```

## 复杂度与适用场景
- 头尾插入/删除 O(1)，按位置遍历 O(n)，随机访问不适合。
- 适合：频繁头尾操作、需要节点地址稳定的场景，或题目明确要求链表。
- 不适合：多数业务场景 `Vec`/`VecDeque` 更快更紧凑；随机访问或缓存友好需求应优先数组/向量结构。
