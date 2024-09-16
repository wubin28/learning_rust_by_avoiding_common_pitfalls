fn main() {
    println!("Rust 避免悬垂指针示例开始运行...");

    let raw_ptr;

    {
        let smart_ptr = Box::new(42);
        raw_ptr = &*smart_ptr as *const i32; // 获取裸指针

        println!("智能指针管理的值: {}", smart_ptr);
        unsafe {
            println!("裸指针指向的值: {}", *raw_ptr);
        }
    } // smart_ptr 在此作用域结束后被销毁

    // 尝试使用 raw_ptr 但编译器并未禁止
    unsafe {
        println!("尝试访问悬垂裸指针的值: {}", *raw_ptr); // 编译通过
    }
}
// Output:
// Rust 避免悬垂指针示例开始运行...
// 智能指针管理的值: 42
// 裸指针指向的值: 42
// 尝试访问悬垂裸指针的值: 1692729408
