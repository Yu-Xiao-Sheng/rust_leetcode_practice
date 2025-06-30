# Rust 刷题必备 API 速查手册

## 📚 目录
- [字符串操作](#字符串操作)
- [Vec (动态数组)](#vec-动态数组)
- [HashMap / HashSet](#hashmap--hashset)
- [BTreeMap / BTreeSet](#btreemap--btreeset)
- [VecDeque (双端队列)](#vecdeque-双端队列)
- [BinaryHeap (堆)](#binaryheap-堆)
- [数值类型操作](#数值类型操作)
- [迭代器常用方法](#迭代器常用方法)
- [常用宏和函数](#常用宏和函数)

---

## 字符串操作

### String vs &str
```rust
// String: 可变的堆分配字符串
let mut s = String::new();
let s = String::from("hello");
let s = "hello".to_string();

// &str: 字符串切片，通常是不可变的
let s: &str = "hello";
let s = &String::from("hello")[..];
```

### 基本操作
```rust
let mut s = String::from("hello");

// 长度和判空
s.len()                    // 字节长度
s.is_empty()              // 是否为空
s.chars().count()         // 字符数量（处理Unicode）

// 添加字符/字符串
s.push('!');              // 添加字符
s.push_str(" world");     // 添加字符串
s += " rust";             // 使用 += 添加

// 插入和删除
s.insert(0, 'H');         // 在索引0处插入字符
s.insert_str(1, "ey");    // 在索引1处插入字符串
s.remove(0);              // 删除索引0处的字符
s.pop();                  // 删除并返回最后一个字符

// 清空和截断
s.clear();                // 清空字符串
s.truncate(5);            // 截断到指定长度
```

### 字符串转换
```rust
let s = "Hello World";

// 转为字符数组
let chars: Vec<char> = s.chars().collect();

// 转为字节数组
let bytes: Vec<u8> = s.bytes().collect();
let bytes = s.as_bytes();  // 返回 &[u8]

// 大小写转换
s.to_lowercase();         // 转为小写
s.to_uppercase();         // 转为大写

// 数字转换
let num: i32 = "123".parse().unwrap();
let s = 123.to_string();  // 数字转字符串
```

### 字符串查找和匹配
```rust
let s = "hello world";

// 查找
s.contains("world");      // 是否包含子串
s.starts_with("hello");   // 是否以指定前缀开始
s.ends_with("world");     // 是否以指定后缀结束
s.find("world");          // 查找子串位置，返回 Option<usize>

// 替换
s.replace("world", "rust"); // 替换所有匹配项
s.replacen("l", "L", 2);   // 替换前n个匹配项

// 分割
s.split(' ').collect::<Vec<&str>>();        // 按字符分割
s.split_whitespace().collect::<Vec<&str>>(); // 按空白字符分割
s.lines().collect::<Vec<&str>>();           // 按行分割
```

### 字符串修剪
```rust
let s = "  hello world  ";

s.trim();                 // 去除两端空白
s.trim_start();           // 去除开头空白
s.trim_end();             // 去除结尾空白
s.trim_matches('#');      // 去除两端指定字符
```

---

## Vec (动态数组)

### 创建和初始化
```rust
// 创建空Vec
let mut v: Vec<i32> = Vec::new();
let mut v = vec![];

// 创建带初始值的Vec
let v = vec![1, 2, 3, 4, 5];
let v = vec![0; 10];      // 创建10个0
let v: Vec<i32> = (1..=5).collect(); // 从迭代器收集

// 指定容量
let mut v = Vec::with_capacity(10);
```

### 基本操作
```rust
let mut v = vec![1, 2, 3];

// 长度和容量
v.len();                  // 元素个数
v.is_empty();            // 是否为空
v.capacity();            // 容量

// 添加元素
v.push(4);               // 末尾添加
v.insert(0, 0);          // 指定位置插入
v.extend(vec![5, 6]);    // 批量添加

// 删除元素
v.pop();                 // 删除并返回最后一个元素
v.remove(0);             // 删除指定索引的元素
v.swap_remove(1);        // 快速删除（与最后元素交换）
v.clear();               // 清空所有元素

// 访问元素
v[0];                    // 直接索引（可能panic）
v.get(0);                // 安全访问，返回Option<&T>
v.get_mut(0);            // 可变访问，返回Option<&mut T>
v.first();               // 第一个元素
v.last();                // 最后一个元素
```

### 数组操作
```rust
let mut v = vec![3, 1, 4, 1, 5];

// 排序
v.sort();                // 升序排序
v.sort_by(|a, b| b.cmp(a)); // 自定义排序（降序）
v.reverse();             // 反转

// 查找
v.binary_search(&3);     // 二分查找（需要已排序）
v.contains(&3);          // 是否包含元素
v.iter().position(|&x| x == 3); // 查找位置

// 切片操作
&v[1..3];                // 切片 [start..end)
&v[..2];                 // 前两个元素
&v[2..];                 // 从索引2开始的所有元素
v.split_at(2);           // 在位置2分割成两部分
```

### 常用算法
```rust
let v = vec![1, 2, 3, 4, 5];

// 聚合操作
v.iter().sum::<i32>();   // 求和
v.iter().max();          // 最大值
v.iter().min();          // 最小值
v.iter().count();        // 计数

// 变换
v.iter().map(|x| x * 2).collect::<Vec<i32>>(); // 映射
v.iter().filter(|&&x| x > 2).collect::<Vec<&i32>>(); // 过滤
v.iter().enumerate();    // 带索引的迭代器
```

---

## HashMap / HashSet

### HashMap 基本操作
```rust
use std::collections::HashMap;

// 创建
let mut map = HashMap::new();
let map: HashMap<String, i32> = [
    ("apple".to_string(), 5),
    ("banana".to_string(), 3),
].iter().cloned().collect();

// 插入和更新
map.insert("key".to_string(), 42);
map.entry("key".to_string()).or_insert(0); // 不存在时插入
map.entry("key".to_string()).and_modify(|e| *e += 1); // 存在时修改

// 访问
map.get("key");              // 返回 Option<&V>
map.get_mut("key");          // 返回 Option<&mut V>
map["key"];                  // 直接访问（可能panic）

// 删除
map.remove("key");           // 删除并返回值
map.clear();                 // 清空

// 查询
map.contains_key("key");     // 是否包含键
map.len();                   // 元素个数
map.is_empty();              // 是否为空

// 遍历
for (key, value) in &map { } // 遍历键值对
for key in map.keys() { }    // 遍历键
for value in map.values() { } // 遍历值
```

### HashSet 基本操作
```rust
use std::collections::HashSet;

let mut set = HashSet::new();

// 插入和删除
set.insert(1);
set.remove(&1);
set.clear();

// 查询
set.contains(&1);
set.len();
set.is_empty();

// 集合操作
let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
let set2: HashSet<i32> = [2, 3, 4].iter().cloned().collect();

set1.union(&set2).cloned().collect::<HashSet<i32>>();        // 并集
set1.intersection(&set2).cloned().collect::<HashSet<i32>>(); // 交集
set1.difference(&set2).cloned().collect::<HashSet<i32>>();   // 差集
```

---

## BTreeMap / BTreeSet

```rust
use std::collections::{BTreeMap, BTreeSet};

// BTreeMap - 有序的键值对集合
let mut btree_map = BTreeMap::new();
btree_map.insert(3, "three");
btree_map.insert(1, "one");
btree_map.insert(2, "two");

// 自动按键排序
for (key, value) in &btree_map {
    println!("{}: {}", key, value); // 输出: 1: one, 2: two, 3: three
}

// 范围查询
btree_map.range(1..3);           // 范围 [1, 3)

// BTreeSet - 有序集合
let mut btree_set = BTreeSet::new();
btree_set.insert(3);
btree_set.insert(1);
btree_set.insert(2);
// 自动排序: [1, 2, 3]
```

---

## VecDeque (双端队列)

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();

// 两端操作
deque.push_front(1);     // 前端添加
deque.push_back(2);      // 后端添加
deque.pop_front();       // 前端删除
deque.pop_back();        // 后端删除

// 访问
deque.front();           // 前端元素
deque.back();            // 后端元素
deque[0];                // 索引访问

// 常用于BFS、滑动窗口等场景
```

---

## BinaryHeap (堆)

```rust
use std::collections::BinaryHeap;

// 默认是最大堆
let mut heap = BinaryHeap::new();

heap.push(3);
heap.push(1);
heap.push(4);

heap.pop();              // 返回最大值
heap.peek();             // 查看最大值但不删除

// 最小堆的实现
use std::cmp::Reverse;
let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
// pop() 会返回 Reverse(1)，即最小值
```

---

## 数值类型操作

### 基本数值类型
```rust
// 整数类型: i8, i16, i32, i64, i128, isize
// 无符号: u8, u16, u32, u64, u128, usize
// 浮点: f32, f64

let x: i32 = 42;
let y: f64 = 3.14;

// 类型转换
x as f64;                // 强制转换
x.into();                // Into trait
```

### 数学操作
```rust
use std::cmp::{min, max};

// 比较
min(a, b);               // 最小值
max(a, b);               // 最大值
a.min(b);                // 方法调用形式
a.max(b);

// 绝对值和符号
x.abs();                 // 绝对值
x.signum();              // 符号 (-1, 0, 1)

// 幂运算
x.pow(2);                // x的2次方
2_i32.pow(10);           // 2的10次方

// 平方根（仅浮点数）
y.sqrt();
```

### 范围和迭代
```rust
// 范围
(1..5).collect::<Vec<i32>>();     // [1, 2, 3, 4]
(1..=5).collect::<Vec<i32>>();    // [1, 2, 3, 4, 5]

// 步长
(0..10).step_by(2).collect::<Vec<i32>>(); // [0, 2, 4, 6, 8]
```

---

## 迭代器常用方法

### 创建迭代器
```rust
let v = vec![1, 2, 3, 4, 5];

v.iter();                // 不可变引用迭代器
v.iter_mut();            // 可变引用迭代器
v.into_iter();           // 获取所有权的迭代器
(1..10);                 // 范围迭代器
```

### 变换方法
```rust
let v = vec![1, 2, 3, 4, 5];

v.iter()
    .map(|x| x * 2)      // 映射变换
    .filter(|&&x| x > 4) // 过滤
    .enumerate()         // 添加索引 (index, item)
    .take(3)             // 取前3个
    .skip(1)             // 跳过前1个
    .rev()               // 反转
    .collect::<Vec<_>>(); // 收集结果
```

### 聚合方法
```rust
let v = vec![1, 2, 3, 4, 5];

v.iter().sum::<i32>();   // 求和
v.iter().product::<i32>(); // 求积
v.iter().max();          // 最大值
v.iter().min();          // 最小值
v.iter().count();        // 计数

// 查找
v.iter().find(|&&x| x > 3);     // 查找第一个满足条件的
v.iter().position(|&x| x == 3); // 查找位置
v.iter().any(|&x| x > 3);       // 是否存在满足条件的
v.iter().all(|&x| x > 0);       // 是否全部满足条件

// 折叠
v.iter().fold(0, |acc, &x| acc + x); // 折叠（类似reduce）
v.iter().reduce(|acc, &x| acc + x);  // 归约
```

---

## 常用宏和函数

### 输入输出
```rust
// 打印
println!("Hello {}", name);
print!("No newline");
eprintln!("Error message"); // 错误输出

// 格式化
format!("Hello {}", name);   // 返回String

// 调试打印
println!("{:?}", vec);       // Debug格式
println!("{:#?}", vec);      // 美化Debug格式
```

### 断言和调试
```rust
assert!(condition);          // 断言
assert_eq!(a, b);           // 相等断言
assert_ne!(a, b);           // 不等断言
debug_assert!(condition);    // 仅在debug模式下的断言

panic!("Something went wrong"); // 主动panic
```

### 条件编译和属性
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MyStruct;

#[allow(dead_code)]         // 允许死代码
#[cfg(test)]                // 仅测试时编译
```

---

## 🎯 刷题常用模板

### 读取输入（在线判题）
```rust
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        // 处理输入...
    }
}
```

### 常用导入
```rust
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, BinaryHeap};
use std::cmp::{min, max, Reverse};
use std::mem;
```

### 快速排序和去重
```rust
let mut v = vec![3, 1, 4, 1, 5];
v.sort();                    // 排序
v.dedup();                   // 去重（需要先排序）

// 或者使用HashSet去重
let unique: Vec<i32> = v.into_iter().collect::<HashSet<_>>().into_iter().collect();
```

---

## 💡 刷题技巧总结

1. **字符串处理**：多用 `chars().collect()` 转为 `Vec<char>`
2. **数组操作**：善用 `sort()`, `binary_search()`, `reverse()`
3. **哈希表**：用 `HashMap` 做缓存，`HashSet` 做去重
4. **双端队列**：`VecDeque` 适合BFS和滑动窗口
5. **堆**：`BinaryHeap` 处理优先级问题
6. **迭代器**：链式调用让代码更简洁
7. **类型转换**：注意 `as`, `into()`, `parse()` 的使用

记住：**熟练掌握这些API是刷题的基础！** 建议多练习，形成肌肉记忆。 