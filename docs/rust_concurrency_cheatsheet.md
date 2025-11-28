# Rust 并发/异步速查笔记

面向有 Java/Python 经验但刚上手 Rust 的同学，记录常见概念、选择理由与小例子。

## Clone 成本怎么判断
- `Copy` 类型（如 `u32`/`bool`/仅含 Copy 字段的小 struct）：`clone()` 只是按位复制，便宜。
- 堆数据容器（`String`/`Vec`/`HashMap`）：`clone()` 会分配并复制底层缓冲区。
- 智能指针：`Rc/Arc` 的 `clone()` 只会引用计数 +1，指向同一对象；`Box` 的 `clone()` 会深拷贝内部值。
- 引用 `&T/&mut T` 的 `clone()` 只是复制指针。
- 不确定时看类型文档/是否实现 `Copy`/是否有 `Drop`；用 `clippy` 的 `clone_on_copy`、`redundant_clone` 等 lint 提醒冗余克隆。

## Rc vs Arc
- `Rc<T>`：单线程共享所有权，非原子计数，轻量，但不是 `Send/Sync`。
- `Arc<T>`：原子计数，可跨线程共享，`Send + Sync`（当 `T` 也满足），成本略高。
- 都有 `Weak` 防环；可变共享要配合 `RefCell`（仅 Rc）或 `Mutex/RwLock`（Arc）。
- 选择：单线程用 Rc；跨线程共享用 Arc。

## Send / Sync 是什么
- `Send`：值的所有权可在线程间安全移动（可传给 `thread::spawn`）。
- `Sync`：共享引用可在线程间安全共享，等价于 `&T: Send`。
- 编译器自动推导：字段都 `Send` ⇒ 结构体 `Send`；字段都 `Sync` ⇒ 结构体 `Sync`。
- 失去 `Send/Sync` 的典型字段：`Rc`、`RefCell`、`Cell`、裸指针。想跨线程共享可改成 `Arc<Mutex<_>>` 等。

## `'static` 生命周期
- 表示“活得和程序一样久”，常见满足方式：
  - 静态数据：字符串字面量 `let s: &'static str = "hi";`
  - 把所有权移进任务/线程：`thread::spawn(move || println!("{s}"))`，线程结束前数据不会被释放。
  - 用 `Arc` 等让数据在引用计数归零前一直存在。
- 不允许捕获短借用进入线程/异步任务，否则会悬垂。

## `move` 闭包的作用
- 改变捕获方式：默认是借用，`move` 会把需要的外部变量按所有权搬进闭包（Copy 类型则是复制）。
- 用途：满足 `'static` 需求（线程/异步任务），或想让闭包独立拥有数据、避免外部再修改。
- 示意：
```rust
let s = String::from("hi");
std::thread::spawn(move || println!("{s}")); // s 所有权被移动进线程
```
- 默认捕获规则与对比：
  - 默认尽量用不可变借用，如果闭包内部需要修改才会升级为可变借用。
  - 只有在确实需要拿所有权（移动或复制 Copy 值）时才会捕获所有权；显式 `move` 强制拿所有权。
```rust
let mut v = vec![1, 2, 3];
// 默认借用，可多次调用，外部还能用 v（受借用规则限制）
let mut f = || {
    v.push(4); // 需要可变借用，闭包是 FnMut
};
f();
v.push(5); // 继续使用 v 没问题

let s = String::from("hello");
let g = || println!("{}", s); // 默认不可变借用，g: Fn
g();
println!("{}", s); // 仍可用 s

let h = move || println!("{}", s); // 拿走所有权，h: FnOnce（可能只调用一次）
h();
// println!("{}", s); // 编译错误：s 已被移动

let x = 1; // Copy
let c = move || println!("{}", x); // 复制到闭包里，外部依然能用
println!("{}", x); // OK
```

## 闭包三兄弟：Fn / FnMut / FnOnce
- 区别（从限制最强到最弱的约束）：  
  - `Fn`: 只读捕获，不可变借用，允许多次调用。  
  - `FnMut`: 可变借用或内部状态修改，需用可变引用调用，允许多次调用。  
  - `FnOnce`: 至少消费了捕获的所有权，只能保证调用一次。
- 推断规则：编译器会让闭包实现能力最强的那个 trait；一旦内部消费了捕获（`drop`/移动），就只能是 `FnOnce`。`move` 会把捕获变成所有权，但未消费时仍可实现 `Fn`/`FnMut`。
- 示例：
```rust
// Fn：只读
let s = String::from("hi");
let f = || println!("{}", s); // 只读借用
call_fn(f);
fn call_fn<F: Fn()>(f: F) { f(); f(); }

// FnMut：可变借用
let mut n = 0;
let mut g = || { n += 1; };
call_fn_mut(&mut g);
fn call_fn_mut<F: FnMut()>(mut f: F) { f(); f(); }

// FnOnce：消费所有权
let v = vec![1, 2, 3];
let h = || drop(v); // v 被消费
call_fn_once(h);
fn call_fn_once<F: FnOnce()>(f: F) { f(); }

// move + 未消费：仍可多次调用
let s = String::from("hello");
let mut k = move || println!("{}", s); // 拿所有权但未消费
call_fn_mut(&mut k); // 可以多次调用
```

## 三种 spawn 的区别
- `std::thread::spawn`：开 OS 线程，真正并行；要求闭包捕获的数据 `Send + 'static`。
- `tokio::spawn`：多线程运行时上的异步任务，任务可能在线程池间移动；要求 `Future: Send + 'static`。
- `tokio::task::spawn_local`：只能在单线程运行时（`#[tokio::main(flavor = "current_thread")]`）使用，任务不跨线程，可持有 `!Send`（如 `Rc`/`RefCell`），但仍需 `'static`。

## 单线程 vs 多线程 Tokio 运行时
- 单线程（`current_thread`）：无跨线程调度/原子操作，延迟和开销低，支持 `spawn_local` 携带 `!Send`；但只能用一个核，长时间阻塞/重计算会卡住所有任务。
- 多线程（默认）：任务在池里 work-stealing，可并行利用多核；需 `Send`，有原子开销，适合有 CPU/混合负载或大量请求。
- 选择策略：
  - 需要 `Rc/RefCell` 等 `!Send`：单线程 + `spawn_local`，或改用 `Arc` 等让任务可 `Send`。
  - 有明显 CPU/阻塞：用多线程，并把阻塞/重计算放 `spawn_blocking`/专用线程池。
  - 纯 I/O、轻计算：二者差距小，单线程延迟略低，压测后决定。

## 例子对照
### 用 Rc 的单线程 Tokio
```rust
#[tokio::main(flavor = \"current_thread\")]
async fn main() {
    let rc = std::rc::Rc::new(1); // !Send
    tokio::task::spawn_local({
        let rc = rc.clone();
        async move { println!(\"{rc}\"); }
    }).await.unwrap();
}
```

### 跨线程共享用 Arc
```rust
use std::sync::Arc;

#[tokio::main] // 默认多线程
async fn main() {
    let shared = Arc::new(vec![1, 2, 3]);
    let s1 = shared.clone();
    tokio::spawn(async move {
        println!(\"{:?}\", s1);
    }).await.unwrap();
}
```

### 线程示例（需要 Send + 'static）
```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let data = Arc::new(String::from(\"hi\")); // Arc 保证跨线程共享且 'static
    let d = data.clone();
    thread::spawn(move || {
        println!(\"{d}\");
    }).join().unwrap();
}
```

## 快速排查/选择清单
- 数据要跨线程移动/共享？需要 `Send`；用 `Arc` + 锁/原子包装可变部分。
- 想在异步里用 `Rc`/`RefCell`？选单线程运行时 + `spawn_local`。
- 任务报 `'static` 不满足？改用 `move` 捕获所有权，或把数据放进 `Arc`/全局。
- 担心 `clone()` 成本？检查类型是否 `Copy`，查看文档；容器通常是深拷贝，`Rc/Arc` 只是计数 +1。

## Box 是什么、何时使用
- 概念：`Box<T>` 是最简单的堆指针，拥有堆上 `T` 的唯一所有权，编译期大小固定为指针大小。
- 作用场景：
  - 将大对象放堆上，栈上只留指针，减少栈占用。
  - 处理递归类型（编译器需要已知大小）：如链表节点 `next: Option<Box<Node>>`。
  - 做特征对象（动态分发）：`Box<dyn Trait>` 让不同具体类型实现同一接口并通过指针调用。
  - 与 `Send/Sync`：`Box<T>` 的并发属性取决于 `T`，`T: Send` 则 `Box<T>: Send`。
- 用法示例：
```rust
// 基本堆分配
let b = Box::new(42);
println!("{}", *b); // 解引用

// 递归类型
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 特征对象
trait Animal { fn speak(&self); }
struct Dog;
impl Animal for Dog { fn speak(&self) { println!("woof"); } }
let a: Box<dyn Animal> = Box::new(Dog);
a.speak();
```
- 何时不用 Box：需要多所有权时用 `Rc/Arc`；需要内部共享可变则配合锁；仅需栈上存储或无需动态分发时直接用值或引用即可。多态但零开销可用泛型，而非 `Box<dyn Trait>`。
