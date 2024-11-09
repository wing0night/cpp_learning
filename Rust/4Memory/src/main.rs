fn main() {
    dynamic_str();
    data_interaction();
    var_scope();

    // 引用相关
    try_quote();
    apply_quote();
    var_quote();
}

// Rust的动态字符串类型String。该类型被分配到堆上，可以动态伸缩
fn dynamic_str(){
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
}

// 变量绑定的数据交互
fn data_interaction(){
    // 存在栈上
    let x = 5;
    let y = x; // 复制值
    println!("x = {}, y = {}", x, y);

    // 存在堆上，动态伸缩
    let s1 = String::from("hello");
    let s2 = s1; // 发生所有权转移
    // s1对这块内存的所有权被转移给s2
    // 可以这样理解String::from("hello")形式的变量绑定：s1持有了通过String::from("hello")创建的字符串的所有权
    println!("{}, world!", s2);

    // 深拷贝clone
    // 对s1的数据进行完全拷贝，不再是默认的所有权的转移
    // 在较为复杂的代码中，会极大的降低程序性能
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

// 变量作用域
// 不可copy的变量类型在main中被函数调用后，将不再存在main中
// 可以理解为变量的所有权被转移，不再存在main中
// println
fn var_scope(){
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// 对不可copy变量的引用和解引用
// 实际上相当于指针的概念，指向被引用的值
fn try_quote(){
    let x = 5;
    let y = &x; // y是一个指向x的引用

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用
}

// 将引用传入函数，而不使其获取所有权
fn apply_quote() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 这里传入s1的引用
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize { // 使用&来表明参数s是一个引用
    s.len()
}

// 传入引用改变原来被引用内容的值
// 不直接通过原变量，只是通过引用也可以改变被引用的值
// 可变引用只能同时存在一个
// 可变引用与不可变引用不能同时存在
fn var_quote() {
    let mut s = String::from("hello"); // 要加一个mut，声明为可变变量
    change(&mut s); // 可变引用类型：&mut
    // 传入的s变量被声明为可变引用类型
    println!("{}", s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 利用大括号手动限制变量作用域
fn limit_scope(){
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;
}
// 新版本中，引用作用域的结束位置从花括号变成最后一次使用的位置
fn new_scope() {
    let mut s = String::from("hello");
 
     let r1 = &s;
     let r2 = &s;
     println!("{} and {}", r1, r2);
     // 新编译器中，r1,r2作用域在这里结束
 
     let r3 = &mut s;
     println!("{}", r3);
 } // 老编译器中，r1、r2、r3作用域在这里结束
   // 新编译器中，r3作用域在这里结束





