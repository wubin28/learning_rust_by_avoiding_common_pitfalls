fn main() {
    println!("Rust 避免悬空指针示例开始运行...");

    let reference;

    {
        let smart_ptr = Box::new(42);
        reference = &*smart_ptr;

        println!("智能指针管理的值: {}", smart_ptr);

        println!("引用指向的值: {}", reference);
    } // smart_ptr 在此作用域结束后被销毁

    // 尝试使用 reference 会导致编译错误
    println!("引用指向的值: {}", reference);
}
// 'cargo run' Output (注释掉第16行):
// Rust 避免悬空指针示例开始运行...
// 智能指针管理的值: 42
// 引用指向的值: 42
//
// 'cargo build' Output (去掉第16行注释):
// error[E0597]: `*smart_ptr` does not live long enough
//   --> src/main.rs:8:21
//    |
// 7  |         let smart_ptr = Box::new(42);
//    |             --------- binding `smart_ptr` declared here
// 8  |         reference = &*smart_ptr;
//    |                     ^^^^^^^^^^^ borrowed value does not live long enough
// ...
// 13 |     } // smart_ptr 在此作用域结束后被销毁
//    |     - `*smart_ptr` dropped here while still borrowed
// ...
// 16 |     println!("引用指向的值: {}", reference);
//    |                                  --------- borrow later used here
//
// For more information about this error, try `rustc --explain E0597`.
// error: could not compile `dangling_danger_rust` (bin "dangling_danger_rust") due to 1 previous error