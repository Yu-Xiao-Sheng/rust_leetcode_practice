use std::mem;

// 情况1：两个变体 - 可以享受null指针优化
#[derive(Debug)]
enum TwoVariants {
    Empty,
    More(Box<i32>),
}

// 情况2：三个变体 - 不能完全享受null指针优化
#[derive(Debug)]
enum ThreeVariants {
    Empty,
    More(Box<i32>),
    Another(Box<i32>),
}

// 情况3：三个变体，但只有一个Empty - 仍然需要tag
#[derive(Debug)]
enum ThreeVariantsOneEmpty {
    Empty,
    First(Box<i32>),
    Second(Box<String>),
}

// 情况4：三个变体，两个Empty类型 - 肯定需要tag来区分两个Empty
#[derive(Debug)]
enum ThreeVariantsTwoEmpty {
    Empty1,
    Empty2, 
    More(Box<i32>),
}

// 情况5：复杂的变体
#[derive(Debug)]
enum ComplexEnum {
    None,
    Small(u8),
    Large(Box<[i32; 1000]>),
}

fn main() {
    println!("=== 内存大小对比 ===");
    
    // 测试各种枚举的内存大小
    println!("两个变体枚举大小: {} bytes", mem::size_of::<TwoVariants>());
    println!("三个变体枚举大小: {} bytes", mem::size_of::<ThreeVariants>());
    println!("三个变体(一个Empty)大小: {} bytes", mem::size_of::<ThreeVariantsOneEmpty>());
    println!("三个变体(两个Empty)大小: {} bytes", mem::size_of::<ThreeVariantsTwoEmpty>());
    println!("复杂枚举大小: {} bytes", mem::size_of::<ComplexEnum>());
    
    println!("\n=== 参考大小 ===");
    println!("Box<i32> 大小: {} bytes", mem::size_of::<Box<i32>>());
    println!("Box<String> 大小: {} bytes", mem::size_of::<Box<String>>());
    println!("u8 大小: {} bytes", mem::size_of::<u8>());
    
    println!("\n=== 实际测试 ===");
    
    // 创建实例测试
    let two_empty = TwoVariants::Empty;
    let two_more = TwoVariants::More(Box::new(42));
    
    let three_empty = ThreeVariants::Empty;
    let three_more = ThreeVariants::More(Box::new(42));
    let three_another = ThreeVariants::Another(Box::new(24));
    
    println!("两个变体实例创建成功");
    println!("三个变体实例创建成功");
    
    // 判断null指针优化是否生效的一个方法是看大小
    // 如果大小等于指针大小，说明有优化
    // 如果大小大于指针大小，说明需要额外的tag
    
    let pointer_size = mem::size_of::<*const ()>();
    println!("\n=== 优化分析 ===");
    println!("指针大小: {} bytes", pointer_size);
    
    if mem::size_of::<TwoVariants>() == pointer_size {
        println!("✅ TwoVariants 享受了null指针优化");
    } else {
        println!("❌ TwoVariants 没有享受null指针优化");
    }
    
    if mem::size_of::<ThreeVariants>() == pointer_size {
        println!("✅ ThreeVariants 享受了null指针优化");
    } else {
        println!("❌ ThreeVariants 没有享受null指针优化 (需要 {} 额外字节)", 
                mem::size_of::<ThreeVariants>() - pointer_size);
    }
} 