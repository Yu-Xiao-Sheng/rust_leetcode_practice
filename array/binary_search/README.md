# 二分查找算法总结

## 核心思想
- 在**有序数组**中查找目标值
- 通过比较中间元素，每次排除一半的搜索空间
- 时间复杂度：O(log n)

## 两种区间定义

### 左闭右闭区间 `[left, right]`
```rust
let mut left = 0;
let mut right = nums.len() - 1;  // 包含右边界

while left <= right {  // 可以相等
    let mid = (left + right) >> 1;
    if nums[mid] < target {
        left = mid + 1;
    } else if nums[mid] > target {
        right = mid - 1;  // 减1
    } else {
        return mid as i32;
    }
}
```

### 左闭右开区间 `[left, right)`
```rust
let mut left = 0;
let mut right = nums.len();  // 不包含右边界

while left < right {  // 不能相等
    let mid = (left + right) >> 1;
    if nums[mid] < target {
        left = mid + 1;
    } else if nums[mid] > target {
        right = mid;  // 不减1
    } else {
        return mid as i32;
    }
}
```

## 关键细节

### 循环条件
- `[left, right]`: `while left <= right`
- `[left, right)`: `while left < right`

### 边界更新
- `[left, right]`: `right = mid - 1`
- `[left, right)`: `right = mid`

### 中间值计算
```rust
// 避免整数溢出
let mid = left + (right - left) / 2;  // 推荐
// 或者
let mid = (left + right) >> 1;        // 位运算，但可能溢出
```

## 常见变体

### 查找第一个等于目标值的元素
```rust
fn binary_search_first(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;  // 即使相等也继续向左找
        }
    }
    
    // left 指向第一个 >= target 的位置
    if left < nums.len() && nums[left] == target {
        left as i32
    } else {
        -1
    }
}
```

### 查找最后一个等于目标值的元素
```rust
fn binary_search_last(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] <= target {
            left = mid + 1;  // 即使相等也继续向右找
        } else {
            right = mid;
        }
    }
    
    // left 指向第一个 > target 的位置，所以最后一个 = target 的位置是 left - 1
    // 重要：必须判断 left > 0，防止数组越界！
    if left > 0 && nums[left - 1] == target {
        (left - 1) as i32
    } else {
        -1
    }
}
```

## 重要理解要点

### 为什么最终结果是 left？

**查找第一个等于目标值：**
- 循环结束时，`left` 指向第一个 `>= target` 的位置
- 如果 `nums[left] == target`，那么 `left` 就是第一个等于目标值的位置

**查找最后一个等于目标值：**
- 循环结束时，`left` 指向第一个 `> target` 的位置
- 所以最后一个 `= target` 的位置是 `left - 1`

### 为什么需要 `left > 0` 的判断？

**关键原因：防止数组越界！**

```rust
// 情况1：空数组
let nums = vec![];
let target = 2;
// 循环结束后 left = 0
// 如果不判断 left > 0，会尝试访问 nums[-1]，导致越界！

// 情况2：所有元素都小于目标值
let nums = vec![1, 1, 1];
let target = 2;
// 循环结束后 left = 3
// 需要访问 nums[2]，但 nums[2] != target

// 情况3：第一个元素等于目标值
let nums = vec![2, 3, 4];
let target = 2;
// 循环结束后 left = 1
// 需要访问 nums[0]，且 nums[0] == target
```

**`left` 的可能值：**
- `left = 0`：空数组或所有元素都小于目标值
- `left = 1`：第一个元素等于目标值
- `left = n`：所有元素都小于等于目标值

### 为什么推荐左闭右开区间？

1. **更简洁**：不需要处理 `left == right` 的情况
2. **更统一**：`left` 始终指向可能的位置
3. **更直观**：循环条件 `while left < right` 更清晰
4. **避免边界问题**：不会出现 `left > right` 的情况

## 关键：if 条件中 `<` 和 `<=` 的选择

### 核心原则：**根据你要找什么来决定**

#### 1. **基础二分查找（找任意一个等于目标值的位置）**
```rust
// 找到就返回，不需要继续
if nums[mid] < target {
    left = mid + 1;
} else if nums[mid] > target {
    right = mid;
} else {
    return mid as i32;  // 找到目标值，直接返回
}
```

#### 2. **查找第一个等于目标值的位置**
```rust
// 关键：即使 nums[mid] == target，也要继续向左找
if nums[mid] < target {
    left = mid + 1;
} else {
    right = mid;  // nums[mid] >= target，继续向左找
}
```

**为什么用 `>=`？**
- 当 `nums[mid] == target` 时，我们不确定这是不是第一个
- 可能左边还有相同的值，所以继续向左搜索
- 最终 `left` 会指向第一个 `>= target` 的位置

#### 3. **查找最后一个等于目标值的位置**
```rust
// 关键：即使 nums[mid] == target，也要继续向右找
if nums[mid] <= target {
    left = mid + 1;  // nums[mid] <= target，继续向右找
} else {
    right = mid;
}
```

**为什么用 `<=`？**
- 当 `nums[mid] == target` 时，我们不确定这是不是最后一个
- 可能右边还有相同的值，所以继续向右搜索
- 最终 `left` 会指向第一个 `> target` 的位置

### 记忆技巧

| 查找目标 | if 条件 | 解释 |
|---------|---------|------|
| 任意一个等于目标值 | `nums[mid] < target` 和 `nums[mid] > target` | 找到就返回 |
| **第一个**等于目标值 | `nums[mid] < target` 和 `nums[mid] >= target` | 相等时继续**向左**找 |
| **最后一个**等于目标值 | `nums[mid] <= target` 和 `nums[mid] > target` | 相等时继续**向右**找 |

### 关键理解

**`<` vs `<=` 的本质区别：**

1. **`<` 的情况**：
   - 当 `nums[mid] == target` 时，**停止搜索**
   - 适用于：找任意一个等于目标值的位置

2. **`<=` 的情况**：
   - 当 `nums[mid] == target` 时，**继续搜索**
   - 适用于：找最后一个等于目标值的位置

3. **`>=` 的情况**：
   - 当 `nums[mid] == target` 时，**继续搜索**
   - 适用于：找第一个等于目标值的位置

### 实际应用示例

```rust
// 示例：nums = [1, 2, 2, 2, 3], target = 2

// 查找第一个 2
// 第一次：mid = 2, nums[2] = 2 == target，但继续向左找
// 第二次：mid = 1, nums[1] = 2 == target，但继续向左找  
// 第三次：mid = 0, nums[0] = 1 < target，left = 1
// 结果：left = 1，第一个 2 在位置 1

// 查找最后一个 2
// 第一次：mid = 2, nums[2] = 2 == target，但继续向右找
// 第二次：mid = 3, nums[3] = 2 == target，但继续向右找
// 第三次：mid = 4, nums[4] = 3 > target，right = 4
// 结果：left = 4，最后一个 2 在位置 3 (left - 1)
```

## 注意事项

### 避免整数溢出
```rust
// ❌ 可能溢出
let mid = (left + right) / 2;

// ✅ 安全写法
let mid = left + (right - left) / 2;
```

### Rust 语法要点
```rust
// 在条件分支中必须使用 return
if nums[mid] == target {
    return mid as i32;  // 不能省略 return
}

// 函数最后一行可以省略 return
-1  // 等价于 return -1;
```

### 边界情况处理
```rust
// 查找第一个时：检查 left < nums.len()
if left < nums.len() && nums[left] == target {
    left as i32
}

// 查找最后一个时：检查 left > 0
if left > 0 && nums[left - 1] == target {
    (left - 1) as i32
}
```

## 应用场景
- 有序数组中查找元素
- 查找插入位置
- 查找边界值
- 数值计算中的二分答案

## 调试技巧
- 打印 `left`, `right`, `mid` 值
- 画图理解区间变化
- 考虑边界情况（空数组、单个元素等）
- 测试各种边界情况

## 完整示例代码

参考 `example.rs` 文件中的实现。

## 运行方式

```bash
# 编译并运行
rustc base_binary_search -o example && ./example

# 或者使用 rust-script（如果有依赖）
rust-script base_binary_search
``` 