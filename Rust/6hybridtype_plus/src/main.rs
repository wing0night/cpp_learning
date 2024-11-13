fn main() {
    // 枚举补充
    // option枚举用来处理空值
    // option枚举的定义(在标准库中就已经被定义，因此不需要在main中再定义一次)
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // 使用option枚举
    let _x: Option<i32> = Some(5);
    let _y: Option<f64> = Some(5.0);
    let _z: Option<&str> = Some("hello");
    // option<T>和T是不同类型，不能加和比较等
    // 只有当使用 Option<i8>（或者任何用到的类型）的时候才需要担心可能没有值
    // 也就是说，Option<T> 是一个安全的空值类型，它可以用来替代 null
    // Option<i8>在经过编译器确认之后才会变成i8
    // 只要一个值不是 Option<T> 类型，你就 可以 安全的认定它的值不为空

    // 关于match的介绍
    // 在表达式不同时运行不同的代码
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);
    
    // 数组
    // 数组分为两类，array 为数组（栈上），Vector 为动态数组（堆上）
    // 两个数组的关系跟 &str 与 String 的关系很像，前者是长度固定的字符串切片，后者是可动态增长的字符串。其实，在 Rust 中无论是 String 还是 Vector，它们都是 Rust 的高级类型：集合类型
    // rust的array数组是固定长度的，其它编程语言的数组往往是可变长度的，与 Rust 中的动态数组 Vector 类似

    // 声明（几种声明方法）
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 类型; 长度
    let a2 = [1, 2, 3, 4, 5]; // 类型可以省略，自动推断
    let a3 = [3; 5]; // 5个3组成的数组
    // 索引方式访问数组元素
    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素
    println!("{:?} {:?}", first, second);

    // 以下会报错
    // 这里的8个是靠copy的，但是String类型是不支持copy的
    // let array = [String::from("rust is good!"); 8];
    // println!("{:#?}", array);
    // 正确的写法：调用std::array::from_fn
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);

    // 数组切片
    let a_slice: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a_slice[1..3]; // 右开
    assert_eq!(slice, &[2, 3]);

    // [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
    

}
