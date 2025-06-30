// 通用的结果验证器
pub struct TestValidator {
    passed: u32,
    failed: u32,
}

impl TestValidator {
    pub fn new() -> Self {
        Self { passed: 0, failed: 0 }
    }
    
    // 手动增加通过计数
    pub fn increment_passed(&mut self) {
        self.passed += 1;
    }
    
    // 手动增加失败计数
    pub fn increment_failed(&mut self) {
        self.failed += 1;
    }
    
    // 验证布尔值结果
    pub fn assert_bool(&mut self, test_name: &str, actual: bool, expected: bool) {
        if actual == expected {
            println!("✅ {}: 期望 {}, 实际 {}", test_name, expected, actual);
            self.passed += 1;
        } else {
            println!("❌ {}: 期望 {}, 实际 {}", test_name, expected, actual);
            self.failed += 1;
        }
    }
    
    // 验证整数结果
    pub fn assert_int(&mut self, test_name: &str, actual: i32, expected: i32) {
        if actual == expected {
            println!("✅ {}: 期望 {}, 实际 {}", test_name, expected, actual);
            self.passed += 1;
        } else {
            println!("❌ {}: 期望 {}, 实际 {}", test_name, expected, actual);
            self.failed += 1;
        }
    }
    
    // 验证向量结果
    pub fn assert_vec(&mut self, test_name: &str, actual: &[i32], expected: &[i32]) {
        if actual == expected {
            println!("✅ {}: 期望 {:?}, 实际 {:?}", test_name, expected, actual);
            self.passed += 1;
        } else {
            println!("❌ {}: 期望 {:?}, 实际 {:?}", test_name, expected, actual);
            self.failed += 1;
        }
    }
    
    // 验证字符串结果
    pub fn assert_string(&mut self, test_name: &str, actual: &str, expected: &str) {
        if actual == expected {
            println!("✅ {}: 期望 '{}', 实际 '{}'", test_name, expected, actual);
            self.passed += 1;
        } else {
            println!("❌ {}: 期望 '{}', 实际 '{}'", test_name, expected, actual);
            self.failed += 1;
        }
    }
    
    // 验证浮点数结果（带精度）
    pub fn assert_float(&mut self, test_name: &str, actual: f64, expected: f64, precision: f64) {
        if (actual - expected).abs() < precision {
            println!("✅ {}: 期望 {:.6}, 实际 {:.6}", test_name, expected, actual);
            self.passed += 1;
        } else {
            println!("❌ {}: 期望 {:.6}, 实际 {:.6}", test_name, expected, actual);
            self.failed += 1;
        }
    }
    
    // 验证任意类型（需要实现 PartialEq 和 Display）
    pub fn assert_eq<T: PartialEq + std::fmt::Display>(&mut self, test_name: &str, actual: T, expected: T) {
        if actual == expected {
            println!("✅ {}: 期望 {}, 实际 {}", test_name, expected, actual);
            self.passed += 1;
        } else {
            println!("❌ {}: 期望 {}, 实际 {}", test_name, expected, actual);
            self.failed += 1;
        }
    }
    
    // 验证任意类型（需要实现 PartialEq 和 Debug）
    pub fn assert_eq_debug<T: PartialEq + std::fmt::Debug>(&mut self, test_name: &str, actual: T, expected: T) {
        if actual == expected {
            println!("✅ {}: 期望 {:?}, 实际 {:?}", test_name, expected, actual);
            self.passed += 1;
        } else {
            println!("❌ {}: 期望 {:?}, 实际 {:?}", test_name, expected, actual);
            self.failed += 1;
        }
    }
    
    // 打印测试统计
    pub fn print_summary(&self) {
        println!("\n=== 测试结果统计 ===");
        println!("通过: {} 个测试", self.passed);
        println!("失败: {} 个测试", self.failed);
        println!("总计: {} 个测试", self.passed + self.failed);
        
        if self.failed == 0 {
            println!("🎉 所有测试通过！");
        } else {
            println!("⚠️  有 {} 个测试失败", self.failed);
        }
    }
    
    // 获取测试统计
    pub fn get_stats(&self) -> (u32, u32) {
        (self.passed, self.failed)
    }
    
    // 重置统计
    pub fn reset(&mut self) {
        self.passed = 0;
        self.failed = 0;
    }
}

// 便捷的测试宏
#[macro_export]
macro_rules! test_case {
    ($validator:expr, $name:expr, $actual:expr, $expected:expr) => {
        $validator.assert_eq($name, $actual, $expected);
    };
}

#[macro_export]
macro_rules! test_case_bool {
    ($validator:expr, $name:expr, $actual:expr, $expected:expr) => {
        $validator.assert_bool($name, $actual, $expected);
    };
}

#[macro_export]
macro_rules! test_case_int {
    ($validator:expr, $name:expr, $actual:expr, $expected:expr) => {
        $validator.assert_int($name, $actual, $expected);
    };
}

#[macro_export]
macro_rules! test_case_vec {
    ($validator:expr, $name:expr, $actual:expr, $expected:expr) => {
        $validator.assert_vec($name, $actual, $expected);
    };
}

// 测试组结构体
pub struct TestGroup {
    name: String,
    validator: TestValidator,
}

impl TestGroup {
    pub fn new(name: &str) -> Self {
        println!("--- {} ---", name);
        Self {
            name: name.to_string(),
            validator: TestValidator::new(),
        }
    }
    
    pub fn validator(&mut self) -> &mut TestValidator {
        &mut self.validator
    }
    
    pub fn print_summary(&self) {
        println!("{} 测试完成", self.name);
        self.validator.print_summary();
    }
    
    pub fn get_stats(&self) -> (u32, u32) {
        self.validator.get_stats()
    }
}

// 测试套件结构体
pub struct TestSuite {
    name: String,
    groups: Vec<TestGroup>,
    total_passed: u32,
    total_failed: u32,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        println!("=== {} ===", name);
        Self {
            name: name.to_string(),
            groups: Vec::new(),
            total_passed: 0,
            total_failed: 0,
        }
    }
    
    pub fn add_group(&mut self, group: TestGroup) {
        let (passed, failed) = group.get_stats();
        self.total_passed += passed;
        self.total_failed += failed;
        self.groups.push(group);
    }
    
    pub fn print_final_summary(&self) {
        println!("\n=== 最终测试结果 ===");
        println!("测试套件: {}", self.name);
        println!("总通过: {} 个测试", self.total_passed);
        println!("总失败: {} 个测试", self.total_failed);
        println!("总计: {} 个测试", self.total_passed + self.total_failed);
        
        if self.total_failed == 0 {
            println!("🎉 所有测试通过！");
        } else {
            println!("⚠️  有 {} 个测试失败", self.total_failed);
        }
    }
} 