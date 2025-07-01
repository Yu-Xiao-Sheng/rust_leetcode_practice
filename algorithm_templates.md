# Rust 常用算法模板汇总

## 📖 目录
- [排序算法](#排序算法)
- [二分查找变体](#二分查找变体)
- [双指针技巧](#双指针技巧)
- [滑动窗口](#滑动窗口)
- [动态规划](#动态规划)
- [回溯算法](#回溯算法)
- [深度优先搜索（DFS）](#深度优先搜索dfs)
- [广度优先搜索（BFS）](#广度优先搜索bfs)
- [树算法](#树算法)
- [图算法](#图算法)
- [贪心算法](#贪心算法)
- [位运算技巧](#位运算技巧)

---

## 排序算法

### 快速排序
```rust
fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}
```

### 归并排序
```rust
fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    let mid = arr.len() / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);
    
    let mut temp = arr.to_vec();
    merge(&mut temp[0..mid], &mut temp[mid..], arr);
}

fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}
```

### 堆排序
```rust
fn heap_sort(arr: &mut [i32]) {
    // 构建最大堆
    for i in (0..arr.len() / 2).rev() {
        heapify(arr, arr.len(), i);
    }
    
    // 提取元素
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}
```

---

## 二分查找变体

### 寻找左边界
```rust
fn left_bound(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    if left < nums.len() && nums[left] == target {
        left as i32
    } else {
        -1
    }
}
```

### 寻找右边界
```rust
fn right_bound(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    if left > 0 && nums[left - 1] == target {
        (left - 1) as i32
    } else {
        -1
    }
}
```

---

## 双指针技巧

### 相向双指针
```rust
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    
    None
}
```

### 快慢指针
```rust
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    
    let mut slow = 0;
    for fast in 1..nums.len() {
        if nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }
    
    (slow + 1) as i32
}
```

---

## 滑动窗口

### 固定大小窗口
```rust
fn max_sum_subarray(nums: &[i32], k: usize) -> i32 {
    if nums.len() < k {
        return 0;
    }
    
    let mut window_sum: i32 = nums[0..k].iter().sum();
    let mut max_sum = window_sum;
    
    for i in k..nums.len() {
        window_sum = window_sum - nums[i - k] + nums[i];
        max_sum = max_sum.max(window_sum);
    }
    
    max_sum
}
```

### 可变大小窗口
```rust
fn min_window_substring(s: &str, t: &str) -> String {
    use std::collections::HashMap;
    
    let s_chars: Vec<char> = s.chars().collect();
    let mut need: HashMap<char, i32> = HashMap::new();
    let mut window: HashMap<char, i32> = HashMap::new();
    
    for c in t.chars() {
        *need.entry(c).or_insert(0) += 1;
    }
    
    let mut left = 0;
    let mut right = 0;
    let mut valid = 0;
    let mut start = 0;
    let mut min_len = usize::MAX;
    
    while right < s_chars.len() {
        let c = s_chars[right];
        right += 1;
        
        if need.contains_key(&c) {
            *window.entry(c).or_insert(0) += 1;
            if window[&c] == need[&c] {
                valid += 1;
            }
        }
        
        while valid == need.len() {
            if right - left < min_len {
                start = left;
                min_len = right - left;
            }
            
            let d = s_chars[left];
            left += 1;
            
            if need.contains_key(&d) {
                if window[&d] == need[&d] {
                    valid -= 1;
                }
                *window.entry(d).or_insert(0) -= 1;
            }
        }
    }
    
    if min_len == usize::MAX {
        String::new()
    } else {
        s_chars[start..start + min_len].iter().collect()
    }
}
```

---

## 动态规划

### 线性DP（最长递增子序列）
```rust
fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    
    let mut dp = vec![1; nums.len()];
    
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    
    *dp.iter().max().unwrap()
}
```

### 区间DP（最长回文子序列）
```rust
fn longest_palindromic_subsequence(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut dp = vec![vec![0; n]; n];
    
    // 单个字符都是回文
    for i in 0..n {
        dp[i][i] = 1;
    }
    
    // 从长度2开始
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if chars[i] == chars[j] {
                dp[i][j] = dp[i + 1][j - 1] + 2;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    
    dp[0][n - 1]
}
```

### 背包DP
```rust
fn knapsack_01(weights: &[i32], values: &[i32], capacity: i32) -> i32 {
    let n = weights.len();
    let cap = capacity as usize;
    let mut dp = vec![vec![0; cap + 1]; n + 1];
    
    for i in 1..=n {
        for w in 1..=cap {
            if weights[i - 1] <= w as i32 {
                dp[i][w] = dp[i - 1][w].max(
                    dp[i - 1][w - weights[i - 1] as usize] + values[i - 1]
                );
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    
    dp[n][cap]
}
```

---

## 回溯算法

### 排列问题
```rust
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();
    let mut used = vec![false; nums.len()];
    
    fn backtrack(
        nums: &[i32],
        path: &mut Vec<i32>,
        used: &mut [bool],
        result: &mut Vec<Vec<i32>>
    ) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }
        
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            
            path.push(nums[i]);
            used[i] = true;
            
            backtrack(nums, path, used, result);
            
            path.pop();
            used[i] = false;
        }
    }
    
    backtrack(&nums, &mut path, &mut used, &mut result);
    result
}
```

### 组合问题
```rust
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();
    
    fn backtrack(
        start: i32,
        n: i32,
        k: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>
    ) {
        if path.len() == k as usize {
            result.push(path.clone());
            return;
        }
        
        for i in start..=n {
            path.push(i);
            backtrack(i + 1, n, k, path, result);
            path.pop();
        }
    }
    
    backtrack(1, n, k, &mut path, &mut result);
    result
}
```

---

## 深度优先搜索（DFS）

### 图的DFS
```rust
use std::collections::HashSet;

fn dfs_graph(graph: &Vec<Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    
    fn dfs(
        node: i32,
        graph: &Vec<Vec<i32>>,
        visited: &mut HashSet<i32>,
        result: &mut Vec<i32>
    ) {
        visited.insert(node);
        result.push(node);
        
        for &neighbor in &graph[node as usize] {
            if !visited.contains(&neighbor) {
                dfs(neighbor, graph, visited, result);
            }
        }
    }
    
    dfs(start, graph, &mut visited, &mut result);
    result
}
```

### 矩阵DFS
```rust
fn dfs_matrix(matrix: &mut Vec<Vec<i32>>, i: i32, j: i32) {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    
    if i < 0 || i >= rows || j < 0 || j >= cols || matrix[i as usize][j as usize] == 0 {
        return;
    }
    
    matrix[i as usize][j as usize] = 0; // 标记为已访问
    
    // 四个方向
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (di, dj) in directions.iter() {
        dfs_matrix(matrix, i + di, j + dj);
    }
}
```

---

## 广度优先搜索（BFS）

### 图的BFS
```rust
use std::collections::{VecDeque, HashSet};

fn bfs_graph(graph: &Vec<Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(node) = queue.pop_front() {
        result.push(node);
        
        for &neighbor in &graph[node as usize] {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    
    result
}
```

### 最短路径BFS
```rust
use std::collections::VecDeque;

fn shortest_path_bfs(matrix: &Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> i32 {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let mut visited = vec![vec![false; cols as usize]; rows as usize];
    let mut queue = VecDeque::new();
    
    queue.push_back((start.0, start.1, 0)); // (row, col, distance)
    visited[start.0 as usize][start.1 as usize] = true;
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    while let Some((row, col, dist)) = queue.pop_front() {
        if row == end.0 && col == end.1 {
            return dist;
        }
        
        for (dr, dc) in directions.iter() {
            let new_row = row + dr;
            let new_col = col + dc;
            
            if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols
                && !visited[new_row as usize][new_col as usize]
                && matrix[new_row as usize][new_col as usize] == 0 {
                
                visited[new_row as usize][new_col as usize] = true;
                queue.push_back((new_row, new_col, dist + 1));
            }
        }
    }
    
    -1 // 无法到达
}
```

---

## 树算法

### 二叉树遍历
```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

// 前序遍历
fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    
    fn preorder(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            result.push(n.val);
            preorder(n.left.clone(), result);
            preorder(n.right.clone(), result);
        }
    }
    
    preorder(root, &mut result);
    result
}

// 中序遍历
fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            inorder(n.left.clone(), result);
            result.push(n.val);
            inorder(n.right.clone(), result);
        }
    }
    
    inorder(root, &mut result);
    result
}

// 后序遍历
fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            postorder(n.left.clone(), result);
            postorder(n.right.clone(), result);
            result.push(n.val);
        }
    }
    
    postorder(root, &mut result);
    result
}
```

---

## 图算法

### Dijkstra算法
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();
    
    dist[start] = 0;
    heap.push(Reverse((0, start)));
    
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        
        for &(v, weight) in &graph[u] {
            if dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
                heap.push(Reverse((dist[v], v)));
            }
        }
    }
    
    dist
}
```

### 并查集
```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 路径压缩
        }
        self.parent[x]
    }
    
    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        
        if px == py {
            return false;
        }
        
        // 按秩合并
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else {
            self.parent[py] = px;
            self.rank[px] += 1;
        }
        
        true
    }
}
```

---

## 贪心算法

### 区间调度
```rust
fn interval_scheduling(mut intervals: Vec<(i32, i32)>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    
    // 按结束时间排序
    intervals.sort_by_key(|&(_, end)| end);
    
    let mut count = 1;
    let mut end = intervals[0].1;
    
    for i in 1..intervals.len() {
        if intervals[i].0 >= end {
            count += 1;
            end = intervals[i].1;
        }
    }
    
    count
}
```

---

## 位运算技巧

### 常用位运算
```rust
// 判断奇偶
fn is_odd(n: i32) -> bool {
    n & 1 == 1
}

// 交换两个数
fn swap_without_temp(a: &mut i32, b: &mut i32) {
    if a != b {
        *a ^= *b;
        *b ^= *a;
        *a ^= *b;
    }
}

// 找到唯一的数（其他数都出现两次）
fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
}

// 计算2的幂
fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

// 计算数字的二进制中1的个数
fn hamming_weight(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
        count += 1;
        n &= n - 1; // 清除最低位的1
    }
    count
}
```

---

## 💡 使用建议

1. **选择合适的算法**：根据问题的时间复杂度要求选择
2. **注意边界条件**：空数组、单元素、越界等情况
3. **模板化思维**：熟悉基本模板，再根据具体问题调整
4. **时空权衡**：考虑时间复杂度和空间复杂度的平衡
5. **代码复用**：将常用的辅助函数抽取出来

---

## 🔍 学习路径

1. **基础排序** → **二分查找** → **双指针**
2. **DFS/BFS** → **动态规划** → **回溯**
3. **树算法** → **图算法** → **高级数据结构**
4. **实践应用**：在具体题目中熟练运用这些模板 