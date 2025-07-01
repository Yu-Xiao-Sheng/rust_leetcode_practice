use std::mem;

// 链表节点结构体
// 包含数据元素和指向下一个节点的链接
struct Node {
    elem: i32,    // 存储的数据
    next: Link,   // 指向下一个节点的链接
}

// 链接枚举 - 这是null指针优化的关键
// Empty变体会被优化为null指针(0x0)
// More变体包含实际的Box指针(非null)
// 这样整个枚举只需要一个指针大小的内存，无需额外的tag
enum Link {
    Empty,              // 空链接，会被优化为null指针
    More(Box<Node>),    // 包含节点的链接，指向堆上的Node
}

// 链表主结构体
// 使用单独的结构体封装Link，提供更好的API设计
pub struct List {
    head: Link,  // 链表头部的链接
}

impl List {
    // 创建新的空链表
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // 向链表头部添加元素
    // 使用mem::replace避免所有权移动问题
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            // 关键：使用mem::replace将self.head替换为Empty，同时获得原值的所有权
            // 这样避免了从借用中移动所有权的错误
            next: mem::replace(&mut self.head, Link::Empty),
        };

        // 将新节点设置为头部
        self.head = Link::More(Box::new(new_node));
    }

    // 从链表头部弹出元素
    pub fn pop(&mut self) -> Option<i32> {
        // 同样使用mem::replace获得head的所有权
        // 同时将head替换为Empty，保证链表状态的一致性
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                // 将next节点作为新的head
                self.head = node.next;
                // 返回当前节点的元素
                Some(node.elem)
            }
        }
    }
}

// 手动实现Drop特征以避免栈溢出
// 如果不手动实现，会产生深度递归调用导致栈溢出
impl Drop for List {
    fn drop(&mut self) {
        // 获取头部链接的所有权，同时将head设置为Empty
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        
        // 使用循环而不是递归来释放节点，避免栈溢出
        // 这是尾递归优化：将递归转换为迭代
        while let Link::More(mut boxed_node) = cur_link {
            // 获取下一个节点，同时清空当前节点的next
            // 这样当boxed_node离开作用域时，只会释放当前节点
            // 而不会继续递归释放后续节点
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node在这里自动drop，但由于next已被清空，不会递归
        }
    }
}

fn main() {
    let mut list = List::new();
    // 创建一个长链表来测试我们的Drop实现
    for i in 0..100000 {
        list.push(i);
    }
    // 显式调用drop，如果没有我们的优化实现，这里会栈溢出
    drop(list);
    println!("长链表成功释放，没有栈溢出！");
}
