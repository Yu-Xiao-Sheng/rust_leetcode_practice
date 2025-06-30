// 测试上层模块引用的示例

// 引入测试验证器模块
mod test_validator;
use test_validator::{TestValidator, test_case_bool, test_case_int};

fn main() {
    println!("=== 测试上层模块引用 ===");
    
    let mut validator = TestValidator::new();
    
    // 简单测试
    test_case_bool!(validator, "布尔值测试", true, true);
    test_case_int!(validator, "整数测试", 42, 42);
    test_case_bool!(validator, "失败测试", true, false);
    
    validator.print_summary();
    
    println!("✅ 模块引用测试完成！");
} 